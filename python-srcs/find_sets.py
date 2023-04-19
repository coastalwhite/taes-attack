from taes.io_trait import TargetIO

def is_miss(time_to_access: int):
    return time_to_access > 15

KEY = [
    0xFA12_3412,
    231,
    124,
    1,
]

def run(io: TargetIO):
    while True:
        for key_word in KEY:
            io.send_word(key_word)

        mask = 0
        for set_nr in range(64):
            time_to_access = io.receive_word()

            if is_miss(time_to_access):
                mask |= (1 << set_nr)
        print("Miss Mask: 0x{:016X}".format(mask))