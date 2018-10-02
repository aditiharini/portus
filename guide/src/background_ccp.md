# CCP Programming Model

Traditionally, congestion control algorithms, and thus the API provided by datapaths, have been designed around the idea of taking some action upon receiving each packet acknowledgement.

However, CCP is built around the idea of moving the core congestion control logic out of the immediate datapath in order to gain programmability. In order to maintain good performance, rather than moving it entirely, we split the implementation of a congestion control algorithm between the datapath and a userspace agent. The datapath component is restricted to a simple LISP-like language and is primarily used for collecting statistics and dictating sending behavior, while the userspace algorithm can be arbitrarily complex and is written in a language like Rust or Python with the support of all their available libraries.
Thus, writing an algorithm in CCP actually involves writing two programs that work in tandem and communicate asynchronously, which requires a slightly different way of thinking about congestion control implementation.

More details can be found in our [SIGCOMM '18 paper](https://people.csail.mit.edu/frankc/pubs/ccp-sigcomm.pdf).
