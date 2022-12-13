"""Day 13 of Advent of Code 2022
"""
from functools import cmp_to_key

def check_order(left: int | list, right: int | list) -> int:
    if isinstance(left, int) and isinstance(right, int):
        return left - right

    left = [left] if isinstance(left, int) else left
    right = [right] if isinstance(right, int) else right
    
    for (i, leftval) in enumerate(left):
        if i >= len(right):
            return 1
        if check_order(leftval, right[i]) < 0:
            return -1
        if check_order(leftval, right[i]) > 0:
            return 1
    if len(left) == len(right):
        return 0

    return -1

if __name__ == "__main__":
    sum_ = 0

    with open("inputs/13.txt", "r") as f:
        inputs = f.read()

    inputs = inputs.split("\n\n")
    signals = []
    for (i, line) in enumerate(inputs):
        items = [eval(l) for l in line.splitlines()]
        left, right = items[0], items[1]
        signals.append(left)
        signals.append(right)
        if check_order(left, right) <= 0:
            sum_ += i + 1
    
    print(sum_)

    signals.append([[2]])
    signals.append([[6]])
    signals.sort(key=cmp_to_key(check_order))

    sum_ = 1
    for i, signal in enumerate(signals):
        if signal == [[2]] or signal == [[6]]:
            sum_ *= i + 1
    print(sum_)
