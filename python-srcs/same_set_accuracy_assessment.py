from taes.io_trait import TargetIO
import random

def run(io: TargetIO):
    random.seed(0x1337)

    miss_time = io.receive_word()
    hit_time  = io.receive_word()

    print(f"Miss Time: {miss_time}")
    print(f"Hit Time: {hit_time}")

    with open('same_set_accuracy_assessment.out', 'w') as f:
        for _ in range(1000):
            offset = random.randint(0,0xF)
            io.send_word(offset)

            tta = io.receive_word()

            if abs(tta - miss_time) < abs(tta - hit_time):    
                f.write('0')
            else:
                f.write('1')