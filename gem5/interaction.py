#! /usr/bin/env python

import sys

import taes.attacks
from taes.gem5_io import Gem5IO

if len(sys.argv) < 2:
    print("[ERROR]: No attack name given")
    exit(2)

attack_name = sys.argv[1]


gem5_io = Gem5IO(in_file="./input.txt", out_file="./output.txt")

taes.attacks.run(gem5_io, attack_name)
