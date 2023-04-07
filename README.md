# T-Table AES Prime+Probe Attack

This repository contains the files necessary to perform the T-Table Prime+Probe
attack on both gem5 and a CW305. For the CW305, it uses the [Pulpino-Top Level
CW305][pulpino-top]. Most code is implemented in Rust.

## Setup

To setup everything first run:

```bash
./setup.sh

# TODO: add the path to your synthesized Pulpino here
cp /path/to/set_associative_cache.bit cw305/
```

[pulpino-top]: https://github.com/coastalwhite/pulpino-top-level-cw305