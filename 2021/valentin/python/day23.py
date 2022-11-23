from typing import Iterable, Optional
from queue import PriorityQueue

INF = 2**31 - 1
Board = list[str]
POINTS = {"A": 1, "B": 10, "C": 100, "D": 1000}
END_CHARS = ["A", "B", "C", "D"]
ROOMS_Y = [3, 5, 7, 9]
HIGHWAY_Y = [i for i in range(1, 12) if i not in ROOMS_Y]
EMPTY_SQUARE = "."


def is_end(board: Board) -> bool:
    end = True
    for j, c in zip(ROOMS_Y, END_CHARS):
        end = end and all((board[x][j] == c for x in range(2, len(board) - 1)))
    # need pairs ranged in A,B,C,D order from left to right
    return end


def to_str(board: Board) -> str:
    return "\n".join(board)


def make_mouv(
    board: Board, srci: int, srcj: int, dsti: int, dstj: int
) -> tuple[Board, int]:
    # compute score
    steps = abs(dstj - srcj) + (
        abs(dsti - srci) if dsti == 1 or srci == 1 else srci + dsti - 2
    )
    frog = board[srci][srcj]
    move_score = steps * POINTS[frog]
    new_board = [[c for c in row] for row in board]
    # update frog position since path is valid
    new_board[dsti][dstj] = frog
    new_board[srci][srcj] = EMPTY_SQUARE
    new_board = ["".join(x) for x in new_board]
    return new_board, move_score


def get_dst_in_room(board: Board, frog: str, dstj: int, imax: int) -> Optional[int]:
    dsti = None
    for i in range(2, imax):
        c = board[i][dstj]
        if c == EMPTY_SQUARE:
            dsti = i
        elif c == frog:
            continue
        else:
            return None
    return dsti


def get_src_in_room(board: Board, j: int, imax: int) -> Optional[int]:
    for i in range(2, imax):
        if board[i][j] != EMPTY_SQUARE:
            return i
    return None


def can_go_without_collide(board: Board, j: int, r: int) -> bool:
    d = 1 if j < r else -1
    return all((board[1][x] == EMPTY_SQUARE for x in range(j + d, r + d, d)))


def allowed_moves(board: Board) -> Iterable[tuple[int, int, int, int]]:
    moves = []
    srcs = []
    imax = len(board) - 1
    # look inside highway here frogs must move into THEIR room
    for j in HIGHWAY_Y:
        if board[1][j] in END_CHARS:
            srcs.append((1, j))

    for j in ROOMS_Y:
        i = get_src_in_room(board, j, imax)
        if i:
            srcs.append((i, j))
    for i, j in srcs:
        frog = board[i][j]
        accessible_rooms = (r for r in ROOMS_Y if can_go_without_collide(board, j, r))
        for dstj in accessible_rooms:
            dsti = get_dst_in_room(board, frog, dstj, imax)
            if dsti and j != dstj:
                moves.append((i, j, dsti, dstj))

        accessible_highway = (
            x for x in HIGHWAY_Y if can_go_without_collide(board, j, x)
        )
        for dstj in accessible_highway:
            if i != 1:
                moves.append((i, j, 1, dstj))

    return moves


def dijkstra(init_board: Board) -> tuple[dict[str, int], dict[str, str]]:
    energy = {to_str(init_board): 0}
    prev = dict()
    pqueue = PriorityQueue()
    pqueue.put((0, init_board))
    while not pqueue.empty():
        _, board = pqueue.get()
        if is_end(board):
            break
        str_game = to_str(board)

        cur_cost = energy[str_game]
        for srci, srcj, dsti, dstj in allowed_moves(board):
            neighbor, edge_cost = make_mouv(board, srci, srcj, dsti, dstj)
            str_nei = to_str(neighbor)
            nei_energy = cur_cost + edge_cost
            if nei_energy < energy.get(str_nei, INF):
                energy[str_nei] = nei_energy
                prev[str_nei] = to_str(board)
                pqueue.put((nei_energy, neighbor))
    return energy, prev


def parser(path) -> list[str]:
    with open(path) as f:
        s = f.read().split("\n")
    return [row for row in s if row]


def part1(path: str):
    board = parser(path)
    energy, prev = dijkstra(board)
    min_end_sg, min_end_e = min(
        filter(lambda x: is_end(x[0].split("\n")), energy.items()),
        key=lambda x: x[1],
    )
    played = [(min_end_sg, min_end_e)]
    igs = to_str(board)
    cur = min_end_sg
    while cur != igs:
        cur = prev[cur]
        played.append((cur, energy[cur]))
    played.reverse()
    for p, e in played:
        print(f"score = {e}")
        print(p)


if __name__ == "__main__":
    # part1('input23.txt')
    # part2 is the same as part1 but with some additional lines
    part1("input23p2.txt")
