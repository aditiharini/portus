# Setup

CCP currently supports writing algorithms in Rust, or (through language bindings) Python. We don't plan on adding new language bindings in the near future, but will gladly accept pull requests!

Using CCP for congestion control on your machine requires two high-level steps, which are detailed in the following sections:

1. Install userspace library (via package manager, or build manually)

2. Install datapath support

Once you've completed these steps, you can immediately run any existing CCP algorithm or write your own algorithm.
