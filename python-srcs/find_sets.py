from io_trait import TargetIO

def is_miss(time_to_access: int):
    return time_to_access > 10

def run(io: TargetIO):
    while True:
        io.send_word(0xFA12_3412);
        io.send_word(231);
        io.send_word(124);
        io.send_word(1);
        
        mask = 0
        for set_nr in range(64):
            time_to_access = io.receive_word()
        
            if is_miss(time_to_access):
                mask |= (1 << set_nr)
        print("Miss Mask: 0x{:016X}".format(mask))