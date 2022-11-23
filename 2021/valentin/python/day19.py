from typing import Set, Tuple, NamedTuple


class Vector3(NamedTuple):
    x: int
    y: int
    z: int

    def point(self, direction: int):
        # tourne la main droite et regarde par rapport a la position initiale
        # sur l axe x initial quelle coordonne a t on
        match direction:
            case 0:
                return self
            case 1:
                return Vector3(self.y, -self.x, self.z)
            case 2:
                return Vector3(-self.x, -self.y, self.z)
            case 3:
                return Vector3(-self.y, self.x, self.z)
            case 4:
                return Vector3(self.z, self.y, -self.x)
            case 5:
                return Vector3(-self.z, self.y, self.x)
            case _:
                assert False

    def rotate(self, angle: int):
        match angle:
            case 0:
                return self
            case 1:
                return Vector3(self.x, -self.z, self.y)
            case 2:
                return Vector3(self.x, -self.y, -self.z)
            case 3:
                return Vector3(self.x, self.z, -self.y)

    def __sub__(self, other):
        return Vector3(self.x - other.x, self.y - other.y, self.z - other.z)

    def __add__(self, other):
        return Vector3(self.x + other.x, self.y + other.y, self.z + other.z)


def vec_from_str(string: str):
    s = [int(i) for i in string.split(",")]
    assert len(s) == 3
    return Vector3(s[0], s[1], s[2])


def parser():
    with open("input19.txt") as f:
        s = f.read()
    scanners = s.split("\n\n")
    scanners = [s.split("\n") for s in scanners]
    scanners = [[vec_from_str(line) for line in s if "," in line] for s in scanners]
    return [s for s in scanners if s]


def get_scan_balises(
    fixed: Set[Vector3], scanner: list[Vector3]
) -> Tuple[Vector3, Set[Vector3]] | Tuple[None, None]:
    for i in range(6):
        for j in range(4):
            transformed = [v.point(i).rotate(j) for v in scanner]
            for ok_point in fixed:
                for point in transformed:
                    dif = ok_point - point
                    moved = {v + dif for v in transformed}
                    if len(moved.intersection(fixed)) >= 12:
                        return (dif, moved)
    return (None, None)


def part1() -> Tuple[Set[Vector3], Set[Vector3]]:
    unknown_scans = parser()
    found_scans = {Vector3(0, 0, 0)}
    found_beacons = set(unknown_scans.pop())
    while unknown_scans:
        scan = unknown_scans.pop()
        s_pos, beacons = get_scan_balises(found_beacons, scan)
        if s_pos is None:
            unknown_scans.insert(0, scan)
        else:
            found_scans.add(s_pos)
            found_beacons.update(beacons)
        print(f"{len(unknown_scans)} scanners left")

    print(len(found_beacons))
    return (found_scans, found_beacons)


def manhattan_dist(v1: Vector3, v2: Vector3) -> int:
    return abs(v1.x - v2.x) + abs(v1.y - v2.y) + abs(v1.z - v2.z)


def part2(scanners: Set[Vector3]):
    m = max(manhattan_dist(v1, v2) for v1 in scanners for v2 in scanners)
    print(m)


if __name__ == "__main__":
    s, _ = part1()
    part2(s)
