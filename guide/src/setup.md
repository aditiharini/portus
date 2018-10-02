# Setup

CCP is written in Rust, and thus requires Rust to be built, even if you are only using other language bindings.

1. Install Rust and the nightly toolchain.

    (Rust and toolchain manager): `curl https://sh.rustup.rs -sSf | sh`

    (Nightly toolchain): `rustup install nightly`

2. Checkout Portus version 0.3.3 (you can use a newer version if one exists, but this is the most up to date version at the time of writing).

    `git checkout tags/v0.3.3`

3. Run `make build && make test-portus`. If you run into any issues, check out this page for resolving common problems.

4. Install language bindings

<details><summary><b>Python</b></summary><p>

-   Install Python dependencies:

    `sudo pip install setuptools setuptools_rust`

-   Build the bindings:

    `cd portus/python && make`

</p></details>

<details><summary><b>C</b></summary><p>

Coming soon...

</p></details>

5. Install datapath support (next section)
