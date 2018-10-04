# AIMD Scheme

The high-level idea is to start with a low cwnd, and then as ACKs are received, probe for more bandwidth by continually increasing the cwnd (additively) until eventually a loss occurs, which signals congestion. We then cut our rate (multiplicatively) and repeat. If you were to graph the congestion window over time of a single flow running this scheme in the prescence of a droptail buffer, it would exhibit the classic "sawtooth" behavior:

[img=sawtooth]

Specifically, we'll use the following algorithm:

-   On each ACK, increase CWND by 1/cwnd (this has the affect of increasing the cwnd by roughly 1 packet per RTT)
-   On each loss, cut CWND by 1/2
