#![cfg_attr(feature = "bench", feature(test))]

use std::collections::HashMap;

extern crate bytes;
extern crate libc;
extern crate nix;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

extern crate ccp_measure_lang;

#[macro_use]
pub mod pattern;
pub mod ipc;
pub mod serialize;

use ipc::Ipc;
use ipc::Backend;
use serialize::Msg;

#[derive(Debug)]
pub struct Error(pub String);

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error(format!("err {}", e))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Error {
        Error(format!("err {}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error(format!("err {}", e))
    }
}

impl From<ccp_measure_lang::Error> for Error {
    fn from(e: ccp_measure_lang::Error) -> Error {
        Error(format!("ccp_measure_lang err: {:?}", e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

use ccp_measure_lang::{Reg, Scope};
impl<T: Ipc> Backend<T> {
    /// Algorithm implementations use send_pattern() to control the datapath's behavior by
    /// calling send_pattern() with:
    /// 1. An initialized backend b. See note in start() for ownership.
    /// 2. The flow's sock_id. IPC implementations supporting addressing (e.g. Unix sockets, which can
    /// communicate with many applications using UDP datapaths)  MUST make the address be sock_id
    /// 3. The control pattern prog to install. Implementations can create patterns using make_pattern!.
    /// send_pattern() will return quickly with a Result indicating whether the send was successful.
    pub fn send_pattern(&self, sock_id: u32, prog: pattern::Pattern) -> Result<()> {
        let msg = serialize::pattern::Msg {
            sid: sock_id,
            num_events: prog.len() as u32,
            pattern: prog,
        };

        let buf = serialize::serialize(msg)?;
        self.send_msg(Some(sock_id as u16), &buf[..])?;
        Ok(())
    }

    pub fn install_measurement(&self, sock_id: u32, src: &[u8]) -> Result<Scope> {
        let (bin, sc) = ccp_measure_lang::compile(src)?;
        let msg = serialize::install_fold::Msg {
            sid: sock_id,
            num_instrs: bin.0.len() as u32,
            instrs: bin,
        };

        let buf = serialize::serialize(msg)?;
        self.send_msg(Some(sock_id as u16), &buf[..])?;
        Ok(sc)
    }
}

pub struct Measurement {
    fields: Vec<u64>,
}

impl Measurement {
    pub fn get_field(&self, field: &String, sc: &Scope) -> Option<u64> {
        sc.get(field).and_then(|r| match r {
            &Reg::Perm(idx, _) => Some(self.fields[idx as usize]),
            _ => None,
        })
    }
}

pub trait CongAlg<T: Ipc> {
    fn name(&self) -> String;
    fn create(
        &mut self,
        control: Backend<T>,
        log: Option<slog::Logger>,
        sock_id: u32,
        start_seq: u32,
        init_cwnd: u32,
    );
    fn measurement(&mut self, log: Option<slog::Logger>, sock_id: u32, m: Measurement);
}

/// Main execution loop of ccp for the static pipeline use case.
/// Blocks "forever".
/// In this use case, an algorithm implementation is a binary which
/// 1. Initializes an ipc backend (depending on datapath)
/// 2. Calls start(), passing the backend b and CongAlg alg.
/// start() takes ownership of b. To use send_pattern() below, clone b first.
///
/// start():
/// 1. listens for messages from the datapath
/// 2. call the appropriate message in alg
///
/// start() will never return (-> !). It will panic if:
/// 1. It receives a pattern or install_fold control message (only a datapath should receive these)
/// 2. The IPC channel fails.
pub fn start<T, U>(b: Backend<T>, log_opt: Option<slog::Logger>) -> !
where
    T: Ipc,
    U: CongAlg<T> + Default,
{
    let mut flows = HashMap::<u32, U>::new();
    for m in b.listen().iter() {
        if let Ok(msg) = Msg::from_buf(&m[..]) {
            match msg {
                Msg::Cr(c) => {
                    if flows.contains_key(&c.sid) {
                        log_opt.as_ref().map(|log| {
                            debug!(log, "re-creating already created flow"; "sid" => c.sid);
                        });

                        let alg = flows.get_mut(&c.sid).unwrap();
                        alg.create(b.clone(), log_opt.clone(), c.sid, c.start_seq, 10 * 1460);
                        continue;
                    }

                    log_opt.as_ref().map(|log| {
                        debug!(log, "creating new flow"; "sid" => c.sid, "start_seq" => c.start_seq, "init_cwnd" => 10 * 1460);
                    });

                    let mut alg = U::default();
                    alg.create(b.clone(), log_opt.clone(), c.sid, c.start_seq, 10 * 1460);
                    flows.insert(c.sid, alg);
                }
                Msg::Ms(m) => {
                    if let Some(alg) = flows.get_mut(&m.sid) {
                        alg.measurement(log_opt.clone(), m.sid, Measurement { fields: m.fields })
                    } else {
                        log_opt.as_ref().map(|log| {
                            debug!(log, "measurement for unknown flow"; "sid" => m.sid);
                        });
                    }
                }
                Msg::Pt(_) | Msg::Fld(_) => {
                    panic!(
                        "The start() listener should never receive a pattern message, \
                                     since it is on the CCP side."
                    )
                }
                _ => continue,
            }
        }
    }

    panic!("The IPC receive channel closed.");
}

#[cfg(test)]
mod test;
