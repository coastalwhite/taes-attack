#! /usr/bin/env python

from taes.cw305_io import CW305IO
from taes import find_sets
# import io_accuracy
from program import RAM
from taes.connection import PulpinoConnection

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

find_sets.run(cw305)
# io_accuracy.run(cw305)