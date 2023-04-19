from taes.io_trait import TargetIO

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

TESTS = [
    "simple_write_read noflush",
    "write_read_cache_line noflush",
    "simple_write_read flush",
    "write_read_cache_line flush",
    "write_evict_read",
    "consequentive_array_write_reads",
    "does_cache",
    "does_flush",
]
MAX_LENGTH = max(map(lambda t: len(t), TESTS))

def run(io: TargetIO):
    print()
    print(bcolors.HEADER + "Tests:" + bcolors.ENDC)
    for t in TESTS:
        print("[" + bcolors.OKBLUE + t + bcolors.ENDC + "]: ", end = '')
        print(' ' * (MAX_LENGTH - len(t)), end = '')
        

        did_succeed = io.receive_word()
        if did_succeed == 1:
            print(bcolors.OKGREEN + "Passed" + bcolors.ENDC)
        else:
            print(bcolors.FAIL + "Failed" + bcolors.ENDC)