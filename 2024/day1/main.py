first = []
second = []
with open("input.txt", "r") as f:
    content = f.read().split("\n")
    for line in content:
        numbers = line.split(" ")
        if len(numbers) > 1:
            first.append(int(numbers[0]))
            second.append(int(numbers[-1]))
    first.sort()
    second.sort()
    distance = 0
    for i, j in zip(first, second):
        distance += abs(i - j)
    print(distance)
