import m5
from pulpino import Pulpino

import sys

if len(sys.argv) < 2:
    print("Usage: ./system.py <path/to/binary>")
    exit(2)

elf_binary = sys.argv[1]

pulpino = Pulpino(
    elf_binary = elf_binary,
    in_file = './input.txt',
)

m5.instantiate()

print("Beginning simulation")
exit_event = m5.simulate()
print("Exiting @ tick {} because {}".format(m5.curTick(), exit_event.getCause()))