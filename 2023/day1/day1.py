import re


def part1():
    # puzzles = [
    #     '1abc2',
    #     'pqr3stu8vwx',
    #     'a1b2c3d4e5f',
    #     'treb7uchet',
    # ]

    sum = 0
    with open('2023/day1/input.txt') as f:
        for puzzle in f:
            nums = re.sub(r'[a-zA-Z\n]', '', puzzle)
            if len(nums) == 0:
                print('no numbers found')
            elif len(nums) == 1:
                num = f'{nums[0]}{nums[0]}'
                sum += int(num)
            else:
                num = nums[::len(nums) - 1]
                sum += int(num)
    print(sum)


# part1()


def part2():
    word_nums = {"zero": "z0o", "one": "o1e", "two": "t2o", "three": "t3e", "four": "f4r",
                 "five": "f5e", "six": "s6x", "seven": "s7n", "eight": "e8t", "nine": "n9e"}
    # puzzles = [
    #     "two1nine",
    #     "eightwothree",
    #     "abcone2threexyz",
    #     "xtwone3four",
    #     "4nineeightseven2",
    #     "zoneight234",
    #     "7pqrstsixteen",
    # ]

    sum = 0
    with open('2023/day1/input.txt') as puzzles:
        for puzzle in puzzles:
            for word, num in word_nums.items():
                puzzle = re.sub(f'({word})', num, puzzle)
            nums = re.sub(r'[a-zA-Z\n]', '', puzzle)
            if len(nums) == 0:
                print('no numbers found')
            elif len(nums) == 1:
                num = f'{nums[0]}{nums[0]}'
                print(num)
                sum += int(num)
            else:
                num = nums[::len(nums) - 1]
                print(num)
                sum += int(num)
            print()
    print(sum)


part2()
