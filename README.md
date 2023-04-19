# T-Table AES Prime+Probe Attack

This repository contains the files necessary to perform the T-Table Prime+Probe
attack on both gem5 and a CW305 with a Pulpino Risc-V Core. This is made so it can
be extended for other programs. For the CW305, it uses the [Pulpino-Top
Level CW305][pulpino-top]. Most of the code is implemented using Rust.

## Setup

To setup everything first run:

```bash
# This should link to the root directory of gem5
# Ensure that you have build the `RISCV` variant of gem5
GEM5_DIR=path/to/gem5
./setup.sh

# TODO: add the path to your synthesized Pulpino here
cp /path/to/set_associative_cache.bit cw305/
```

## Running attacks

To run an attack use the `./run.sh` script in the `cw305` or `gem5` directory.
Supply an argument is the name of the attack in the `attacks` directory. The
attacks `io_accuracy`, `test_cache` and `find_sets` are provided. For example,
to run the `find_sets`.

```bash
# For the CW305 (ensure it is connected)
cd cw305
./run.sh find_sets

# For gem5
cd gem5
./run.sh find_sets
```

## Adding a new attack

To add a new attack, it is important to follow a couple of steps:

1. Copy over the `att/io_accuracy` into a `att/new_attack`.
2. Rename the project `name` in the `att/new_attack/Cargo.toml` to `new_attack`.
3. Copy over `python-srcs/io_accuracy.py` to `python-srcs/new_attack.py`.
4. Add an `import` and `match case` in the `python-srcs/attacks.py` with your
      attack.

At this point you can start adjusting the `att/new_attack/src/main.rs` to
contain the code for you attack. You can use the `IO::read_word()` and
`IO::write_word(w)` functions to write to the python interface independent of
whether we are using gem5 or the cw305. We can similarly use the
`io.receive_word()` and `io.send_word()` operations in the
`python-srcs/new_attack.py` to receive and send data to the target. Note that
all these operations are blocking.

> **NOTE**: It is important to use a `snake_case` name for your attack as
> Rust does not deal very well with the name otherwise.

## Content Explanation

This repository contains a couple of folders. The following figure explains
what each directory is for.

```
att:
    The source code of the binaries for gem5 or cw305 to run

cw305:
    The code specific to the CW305. This include the IO and setup code.
    
gem5:
    The code specific to gem5. This include the IO and setup code.

io_trait:
    Defines an rust trait that is used by the `att` source code to generalize
    over the IO for the CW305 and gem5.

model:
    Defines a modelled T-Table AES Prime+Probe attack. This is a very fact
    model that can provide statistics on noise and queries needed.

pulpino-top-level-cw305:
    A git submodule that takes care of the CW305.

python-srcs:
    All the code needed to coordinate the attacks and generalize between the CW305 and gem5.

taes:
    The implementation of T-Table AES. This only contains the forward function which is what the attack is focussed on.
```

[pulpino-top]: https://github.com/coastalwhite/pulpino-top-level-cw305