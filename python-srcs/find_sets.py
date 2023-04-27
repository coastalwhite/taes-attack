from random import randint
from datetime import datetime
from math import log2,ceil,floor

from taes.io_trait import TargetIO

NUM_SETS = 64
QUERIES = 0
DYN_NOISE_REPEAT = 8
KEY = bytearray("supersecretkey!!".encode())

def random_word():
    return "{:02X}{:02X}{:02X}{:02X}".format(
        randint(0, 255),
        randint(0, 255),
        randint(0, 255),
        randint(0, 255),
    )

def get_ct(pt):
    ct = []
    for i in range(16):
        ct.append(pt[i] ^ KEY[i])
    return ct

class Target:
    LOG_TIME=True

    def __init__(self, io: TargetIO) -> None:
        self.io = io

    def write_ct(self, ct):
        for i in range(4):
            word = int.from_bytes(ct[i*4:(i+1)*4], byteorder='big')
            self.io.send_word(word)

    def req_times(self, ct):
        self.write_ct(ct)
        
        times = []
        for _ in range(64):
            times.append(self.io.receive_word())
        
        return times
        
    def get_mask(self, ct):
        global QUERIES

        QUERIES += 1
        start_time = datetime.now()

        times = self.req_times(ct)
        mask = self.times_to_mask(times)

        if self.LOG_TIME:
            end_time = datetime.now()
            print(
                "Finished Query with Mask: {:016X} (in {}s)".format(
                    mask, round((end_time - start_time).total_seconds(), 1)
                )
            )

        return mask

    def is_miss(self, tta):
        return tta > 15

    def times_to_mask(self, times):
        mask = 0
        for i, t in enumerate(times):
            if self.is_miss(t):
                mask |= 1 << (i % 64)
        return mask

def mask_to_freq(mask):
    freq = [0] * NUM_SETS
    for i in range(NUM_SETS):
        if mask & 1 == 1:
            freq[i] = 1
        mask >>= 1
    return freq

def get_mask(target, pt):
    if DYN_NOISE_REPEAT <= 1:
        return target.get_mask(get_ct(pt))

    freqs = [0] * 64
    for i in range(DYN_NOISE_REPEAT):
        freq = mask_to_freq(target.get_mask(get_ct(pt)))
        for j in range(64):
            freqs[j] += freq[j]

    mask = 0
    for i in range(64):
        if not (freqs[i] == DYN_NOISE_REPEAT or freqs[i] == 0):
            print("[INFO]: Detected Noise")

        if freqs[i] > ceil(DYN_NOISE_REPEAT / 2):
            mask |= 1 << i

        if freqs[i] >= floor(DYN_NOISE_REPEAT / 2) and freqs[i] <= ceil(
            DYN_NOISE_REPEAT / 2
        ):
            print("[ERROR]: Unrecoverable noise")
            exit(0)

    print("Decided on mask: {:016X}".format(mask))

    return mask

def run(io: TargetIO):
    target = Target(io)
    get_mask(target, [0] * 16)

    mappings = dict()

    pt = [0] * 16
    while len(mappings) != 16:
        for i in range(16):
            pt[i] = randint(0, 255)

        mask = get_mask(target, pt)

        for i in range(16):
            if i in mappings:
                continue

            for j in range(0, 256):
                if j == pt[i]:
                    continue

                updated_pt = pt[:]
                updated_pt[i] = j
                updated_mask = get_mask(target, updated_pt)

                diff_mask = mask ^ updated_mask

                if diff_mask == 0:
                    continue

                # This triggers in two cases
                # 1. It jumps from a shared spot to a individual spot
                # 2. It jumps from a individual spot to a shared spot
                if diff_mask.bit_count() == 1:
                    bit_idx = int(log2(diff_mask))

                    # If it jumped from a individual spot, we know that spot belonged to
                    # this byte
                    if diff_mask & mask != 0:
                        mappings[i] = bit_idx ^ (pt[i] >> 2)
                        print(
                            "Found byte {i} to be in set {set_idx}".format(
                                i=i, set_idx=mappings[i]
                            )
                        )
                        break

                    # If it jumped to a individual spot, we know that spot belongs to
                    # this byte
                    if diff_mask & updated_mask != 0:
                        mappings[i] = bit_idx ^ (updated_pt[i] >> 2)
                        print(
                            "Found byte {i} to be in set {set_idx}".format(
                                i=i, set_idx=mappings[i]
                            )
                        )
                        break

                # This triggers when it jumps from an individual spot to another
                # individual spot
                if diff_mask.bit_count() == 2:
                    original_bit = diff_mask & mask
                    assert original_bit.bit_count() == 1

                    bit_idx = int(log2(original_bit))
                    mappings[i] = bit_idx ^ (pt[i] >> 2)
                    print(
                        "Found byte {i} to be in set {set_idx}".format(i=i, set_idx=mappings[i])
                    )
                    break

    for i in range(16):
        m = mappings[i]

        start = m * 4
        end = start + 3

        if KEY[i] < start or KEY[i] > end:
            print("WRONG. off by {}".format(min(abs(KEY[i] - start), abs(KEY[i] - end))))
        else:
            print("{:02}: {:02X}-{:02X} (correct = {:03} '{}')".format(
                i, start, end, KEY[i], chr(KEY[i])
            ))

    print("")
    print("Queries: {}".format(QUERIES))

    return mappings