#! /usr/bin/env python

import sys

import taes.attacks
from taes.cw305_io import CW305IO
from program import RAM
from taes.connection import PulpinoConnection

if len(sys.argv) < 2:
    print("[ERROR]: No attack name given")
    exit(2)

attack_name = sys.argv[1]

bitpath = "./set_associative_cache.bit"
pulpino = PulpinoConnection(bitpath, force = True)

if not pulpino.get_raw().fpga.isFPGAProgrammed():
    print("Fpga Not Programmed")
    exit(1)

pulpino.reset()
pulpino.program(0x0, RAM)

# Stop Programming
pulpino.stop_programming()

# Entry Address
pulpino.send_word(0x0)

cw305 = CW305IO(pulpino)

taes.attacks.run(cw305, attack_name)