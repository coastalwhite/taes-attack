#! /usr/bin/env python

from gem5_io import Gem5IO
# import io_accuracy
import find_sets

gem5_io = Gem5IO(in_file = './input.txt', out_file = './output.txt')

# io_accuracy.run(gem5_io)
find_sets.run(gem5_io)
