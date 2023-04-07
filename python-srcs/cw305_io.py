from io_trait import TargetIO

class CW305IO(TargetIO):
    def __init__(self, pulpino) -> None:
        super()
        self.pulpino = pulpino

    def send_word(self, w):
        self.pulpino.send_word(w)

    def receive_word(self):
        return self.pulpino.receive_word()