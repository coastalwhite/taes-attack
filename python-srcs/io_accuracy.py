from io_trait import TargetIO

def checked_add(io: TargetIO, a, b):
    io.send_word(a)
    io.send_word(b)

    assert io.receive_word() == (a + b) & 0xFFFF_FFFF

def run(io: TargetIO):
    checked_add(io, 0, 0)
    checked_add(io, 0xFFFF_FFFF, 1)
    checked_add(io, 20, 22)
    checked_add(io, 1337, 42)
    checked_add(io, 0x1234_5678, 0x9ABC_DEF0)

    checked_add(io, 0x0000_0000, 0x0011_2233)
    checked_add(io, 0x0000_0000, 0x4455_6677)
    checked_add(io, 0x0000_0000, 0x8899_AABB)
    checked_add(io, 0x0000_0000, 0xCCEE_FF00)
    checked_add(io, 0xFEDC_BA98, 0x0000_0000)