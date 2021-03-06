from typing import List
from sys import argv

def row_generator(s: List[str]):
    """
    Useable for any length of s, but only up to 127 will fit in the Repl terminal
    and, even then, the terminal must take up most of the screen.
    """
    count = 1
    digits = max([len(i) for i in s])
    separators = [2**i + 1 for i in range(1, 32)]
    
    lines = 0
    temp = 0
    for i in separators:
        if len(s) == temp:
            break
        elif len(s) < temp:
            break
        lines += 1
        temp += i

    # Clear the old numbers.txt
    with open(argv[2], "w") as f:
        f.write("")
    
    # Log the rows in numbers.txt
    f = open(argv[2], "a")

    for i in s:
        if count in separators:
            f.write("{}".format(i).rjust(digits))
            f.write("\n")
            separators.remove(count)
            count = 1
            lines -= 1
        f.write("{}".format(i).rjust(digits))
        buffer(lines, digits, f)
        count += 1

    f.close()


def buffer(lines: int, digits: int, f):
    for _ in range(1, 2**lines):
        f.write("".rjust(digits))


if __name__ == "__main__":
    with open(argv[1], "r") as f:
        s = f.read().split(" ")

    print("Generating rows...")
    row_generator(s)
    print("Generating complete.")
