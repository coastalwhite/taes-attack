from time import sleep
from taes.io_trait import TargetIO

START_LINE = "*** START"
END_LINE = "*** END"

HEX_TABLE = [
    ord("0"),
    ord("1"),
    ord("2"),
    ord("3"),
    ord("4"),
    ord("5"),
    ord("6"),
    ord("7"),
    ord("8"),
    ord("9"),
    ord("A"),
    ord("B"),
    ord("C"),
    ord("D"),
    ord("E"),
    ord("F"),
]


class Gem5IO(TargetIO):
    def __init__(self, in_file, out_file) -> None:
        super()

        self.in_file_path = in_file
        self.out_file_path = out_file

        self.start_line = None
        while self.start_line == None:
            out_file = open(self.out_file_path, "r")
            lines = out_file.readlines()
            out_file.close()

            self.start_line = next(
                (i for i, line in list(enumerate(lines)) if START_LINE in line), None
            )

            sleep(0.1)

        self.offset_line = self.start_line + 1

    def send_word(self, w):
        bs = "{:08X}".format(w).encode()
        bs = bytearray(bs)
        # bs.append(0)

        in_file = open(self.in_file_path, "ab")
        in_file.write(bs)
        in_file.close()

    def receive_word(self):
        while True:
            out_file = open(self.out_file_path, "r")
            lines = out_file.readlines()
            out_file.close()

            # If the `\n` is added to the end of the line
            if self.offset_line < len(lines) and lines[self.offset_line].endswith('\n'):
                break

        line = lines[self.offset_line].strip()
        assert len(line) == 8

        self.offset_line += 1

        return int(line, 16)