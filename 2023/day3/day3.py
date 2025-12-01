def debug(*s):
    p = ", ".join(map(str, s))
    print("===start===")
    print(format(p))
    print("===end===")


def is_valid(f, i, j, capture):
    # check left
    before = j - len(capture) - 1
    if before >= 0:
        if f[i][before] != ".":
            print(capture, end="+")
            # debug("before", capture, f[i][before])
            return True

    # check right
    if f[i][j] != ".":
        print(capture, end="+")
        # debug("after", capture, f[i][j])
        return True
    check_width = len(capture) + 2

    # check above
    if i > 0:
        above = f[i - 1][before : j + 1]
        if above != "." * check_width and above != "":
            print(capture, end="+")
            # debug("above", f'{capture}', above, check_width)
            return True

    # check below
    if i < len(f) - 1:
        below = f[i + 1][before : j + 1]
        if below != "." * check_width and below != "":
            print(capture, end="+")
            # debug('below', f'{capture}', below, check_width)
            return True
    return False


lines = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
]

unused = ""
count = 0
with open("2023/day3/input.txt", "r") as f:
    lines = f.readlines()
    if lines:
        sum = 0
        for i, line in enumerate(lines):
            count += 1
            if count == 6:
                break
            capture = ""
            for j, char in enumerate(line):
                if char.isdigit():
                    capture += char
                else:
                    if capture != "":
                        if is_valid(lines, i, j, capture):
                            sum += int(capture)
                    capture = ""
        debug(" SUM:", sum)
