GO_LEFT = ">"
GO_DOWN = "v"
EMPTY = "."


def parser(filename: str) -> list[list[str]]:
    with open(filename) as f:
        return [[y for y in x] for x in f.read().split("\n") if x]


def do_one_step(init_puzzle: list[list[str]]) -> tuple[bool, list[list[str]]]:
    has_changed = False
    puzzle = [[y for y in x] for x in init_puzzle]
    len_i = len(puzzle)
    len_j = len(puzzle[0])
    # one pass to move > to the left
    for i in range(len_i):
        for j in range(len_j):
            cur = init_puzzle[i][j]
            j_nex = (j + 1) % len_j
            nex = init_puzzle[i][j_nex]
            if cur == GO_LEFT and nex == EMPTY:
                puzzle[i][j_nex] = GO_LEFT
                puzzle[i][j] = EMPTY
                has_changed = True
    init_puzzle = [[y for y in x] for x in puzzle]
    # second pass to move v down
    for j in range(len_j):
        for i in range(len_i):
            cur = init_puzzle[i][j]
            i_nex = (i + 1) % len_i
            nex = init_puzzle[i_nex][j]
            if cur == GO_DOWN and nex == EMPTY:
                puzzle[i_nex][j] = GO_DOWN
                puzzle[i][j] = EMPTY
                has_changed = True
    return has_changed, puzzle


def part1():
    # puzzle = parser("input25.txt")
    puzzle = parser("input25.txt")
    print(len(puzzle), len(puzzle[0]))
    has_changed = True
    count = 0
    # print("Initial state:")
    # print("\n".join(("".join(x) for x in puzzle)))
    while has_changed:
        has_changed, puzzle = do_one_step(puzzle)
        count += 1
        # print(f"After {count} steps:")
        # print("\n".join(("".join(x) for x in puzzle)))
    print(f"answer = {count}")


if __name__ == "__main__":
    part1()
