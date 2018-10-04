# Existing Algorithms

We have implemented the following algorithms for use with CCP. You can immediately run these on any datapath with CCP support. If you have implemented a new algorithm and would like it to be included in this list, please send a pull request.

-   [BBR](https://github.com/ccp-project/bbr)
-   [Copa](https://github.com/venkatarun95/ccp_copa)
-   [Nimbus](https://github.com/ccp-project/nimbus)
-   [Reno and Cubic](https://github.com/ccp-project/generic-cong-avoid)

Once portus has been installed, you can run an algorithm on CCP as follows:

1. Clone:

    `git clone [algorithm] && cd [algorithm]`

2. Build:

    `cargo +nightly build`

3. Run:

    `sudo ./target/release/[algorithm] --help`

Once CCP is running, any sockets that set the `TCP_CONGESTION` sockopt to `"ccp"` will communicate congestion control information with the userspace CCP agent and will thus use this algorithm implementation. Some applications such as `iperf` conveniently allow this to be set directly from the command line. Add `-Z ccp` for `iperf`(v2) or `-C ccp` for `iperf3`.
