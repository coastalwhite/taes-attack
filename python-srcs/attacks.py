from taes.io_trait import TargetIO
from taes import (
    find_sets,
    io_accuracy,
    test_cache,
    read_registers,
    accuracy_assessment,
    same_set_accuracy_assessment,
)

def run(io: TargetIO, attack_name: str):
    match attack_name:
        case "test_cache":
            test_cache.run(io)
        case "io_accuracy":
            io_accuracy.run(io)
        case "find_sets":
            find_sets.run(io)
        case "accuracy_assessment":
            accuracy_assessment.run(io)
        case "same_set_accuracy_assessment":
            same_set_accuracy_assessment.run(io)
        case _:
            print("[ERROR]: Attack name '{}' not known".format(attack_name))
            exit(2)