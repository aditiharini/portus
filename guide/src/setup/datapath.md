# Datapath Support

In order to collect measurements about the network and actually affect the sending behavior of flows, the userspace agent must communicate with the transport layer (datapath). We have designed a common API for the communication between userspace agent and datapath and implemented this API in a `C` library called `libccp`. We have integrated this library into three popular datapaths and provide instructions for installing our modifications below.

The Linux Kernel datapath integration has received the most use and thus is the most stable, but we have successfully run many of the same experiments and tests on the other datapaths as well.

If you would like to use a datapath that is not listed below, check out the [documentation for libccp](../libccp/index.md) for instructions on adding your own support. In most cases, this should be quite easy, and mainly involves telling libccp where to find the necessary variables and function handlers.

<br />
<details><summary><b>Linux (kernel module)</b></summary><p>

Clone our kernel module:

`git clone https://github.com/ccp-project/ccp-kernel.git`

Fetch submodules:

`git submodule update --init --recursive`

Build:

`cd ccp-kernel && make`

Install: (provide `ipc=0` to use netlink sockets):

`sudo ./ccp_kernel_load ipc=0`

</p></details>
<br />

<details><summary><b>mTCP / DPDK (fork)</b></summary><p>

Clone our fork:

`git clone https://github.com/ccp-project/ccp-mtcp.git`

Follow the instructions in `REAMDE.md` for building mTCP as normal (and for building DPDK first, if you haven't done so already).

More detailed instructions coming soon.

</p></details>
<br />

<details><summary><b>Google QUIC (patch)</b></summary><p>

Our patch currently lives at [https://github.com/ccp-project/ccp-quic](https://github.com/ccp-project/ccp-quic)

Follow the instructions in `README.md` for applying the patch.

More specific instructions for getting QUIC setup from scratch coming soon.

</p></details>
<br />
