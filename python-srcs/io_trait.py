class TargetIO:
    def send_word(self, w):
        raise NotImplementedError()

    def receive_word(self):
        raise NotImplementedError()