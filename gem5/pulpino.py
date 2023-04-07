import m5
from m5.objects import *

NUM_SETS = 64
ENTRY_SIZE = 16

class L1Cache(Cache):
    assoc = 2
    tag_latency = 1
    data_latency = 2
    response_latency = 2
    mshrs = 4
    tgts_per_mshr = 20

    def connectCPU(self, cpu):
        # need to define this in a base class!
        raise NotImplementedError

    def connectBus(self, bus):
        self.mem_side = bus.cpu_side_ports


class L1ICache(L1Cache):
    size = "16kB"

    def connectCPU(self, cpu):
        self.cpu_side = cpu.icache_port


class L1DCache(L1Cache):
    size = "{}B".format(NUM_SETS * ENTRY_SIZE * L1Cache.assoc)
    addr_ranges = [AddrRange(start = 0x10_0000, end = 0x10_8000)]
    replacement_policy = FIFORP()

    def connectCPU(self, cpu):
        self.cpu_side = cpu.dcache_port

class Pulpino:
    def __init__(self, elf_binary, in_file) -> None:
        system = System()

        system.cache_line_size = ENTRY_SIZE

        system.clk_domain = SrcClockDomain()
        system.clk_domain.clock = "100MHz"
        system.clk_domain.voltage_domain = VoltageDomain()

        system.mem_mode = "timing"

        system.mem_ranges = [AddrRange("512MB")]

        system.membus = SystemXBar()

        cpu = Riscv32TimingSimpleCPU()

        system.cpu = cpu

        system.cpu.icache = L1ICache()
        system.cpu.icache.connectCPU(system.cpu)

        system.cpu.dcache = L1DCache()
        system.cpu.dcache.connectCPU(system.cpu)

        system.cpu.icache.connectBus(system.membus)
        system.cpu.dcache.connectBus(system.membus)

        system.cpu.createInterruptController()

        system.cpu.function_trace_start = 0
        system.cpu.function_trace = True

        system.system_port = system.membus.cpu_side_ports

        memory = SimpleMemory()
        memory.range = system.mem_ranges[0]

        memory_latency_clock_cycles = 1
        proc_freq = 10E8
        memory_latency_ns = (10E9 / proc_freq) * memory_latency_clock_cycles
        memory.latency = '{}ns'.format(memory_latency_ns)

        system.mem_ctrl = memory.controller()
        system.mem_ctrl.port = system.membus.mem_side_ports

        system.workload = SEWorkload.init_compatible(elf_binary)

        process = Process()

        process.cmd = [elf_binary]
        process.input = in_file

        system.cpu.workload = process
        system.cpu.createThreads()

        root = Root(full_system=False, system=system)
