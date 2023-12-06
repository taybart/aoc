from functools import reduce


def part1():
    # input = [
    #     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    #     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    #     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    #     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    #     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    # ]

    MAX_RED = 12
    MAX_GREEN = 13
    MAX_BLUE = 14

    valid_games = []
    with open('2023/day2/input.txt') as f:
        for i, line in enumerate(f):
            game = line.split(":")[1].strip()
            game = game.split(";")
            game = [x.strip() for x in game]
            print(f'game {i+1}')
            valid = True
            for round in game:
                if not valid:
                    break
                round = round.split(",")
                round = [x.strip() for x in round]
                print('\t', round)
                for x in round:
                    [amount, color] = x.split(" ")
                    if color == "red" and int(amount) > MAX_RED:
                        valid = False
                        print('GAME INVALID')
                        break
                    if color == "green" and int(amount) > MAX_GREEN:
                        print('GAME INVALID')
                        valid = False
                        break
                    if color == "blue" and int(amount) > MAX_BLUE:
                        print('GAME INVALID')
                        valid = False
                        break
            if valid:
                valid_games.append(i + 1)
    print(sum(valid_games))


def part2():
    total = 0

    # test = [
    #     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    #     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    #     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    #     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    #     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    # ]
    with open('2023/day2/input.txt') as f:
        for i, line in enumerate(f):
            game = line.split(":")[1].strip()
            game = game.split(";")
            game = [x.strip() for x in game]
            print(f'game {i+1}')
            max = [0, 0, 0]
            for round in game:
                round = round.split(",")
                round = [x.strip() for x in round]
                print('\t', round)
                for x in round:
                    [amount, color] = x.split(" ")
                    match color:
                        case "red":
                            if int(amount) > max[0]:
                                max[0] = int(amount)
                        case "green":
                            if int(amount) > max[1]:
                                max[1] = int(amount)
                        case "blue":
                            if int(amount) > max[2]:
                                max[2] = int(amount)
            print(max)
            total += reduce(lambda x, y: x * y, max)
    print(total)
