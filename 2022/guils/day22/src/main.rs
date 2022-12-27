use itertools::Itertools;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Coord = (i32, i32);
type Coord3D = (i32, i32, i32);
type Matrix3D = (Coord3D, Coord3D, Coord3D);
type Map = HashMap<Coord, char>;
type FaceRotMap = HashMap<i32, (Coord3D, (Coord3D, Coord3D))>;
type FaceGraph = HashMap<(i32, usize), (i32, usize)>;
const DIRS: [Coord; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Copy, Debug, Clone)]
enum Move {
    Fwd(i32),
    L,
    R,
}
type Path = Vec<Move>;

fn read_input() -> (Map, Path) {
    let (m, p) = INPUT.split("\n\n").collect_tuple().unwrap();
    let mut map = Map::new();
    for (l, line) in m.split('\n').enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != ' ' {
                map.insert((l as i32, c as i32), ch);
            }
        }
    }
    let mut path = Path::new();
    path.push(Move::Fwd(0));
    for ch in p.trim().chars() {
        let last = path.len() - 1;
        if ch.is_ascii_digit() {
            if let Move::Fwd(i) = path[last] {
                path[last] = Move::Fwd(i * 10 + ch.to_digit(10).unwrap() as i32);
            } else {
                path.push(Move::Fwd(ch.to_digit(10).unwrap() as i32));
            }
        } else if ch == 'R' {
            path.push(Move::R);
        } else if ch == 'L' {
            path.push(Move::L);
        } else {
            unreachable!();
        }
    }
    (map, path)
}

fn walk(map: &Map, path: &Path) -> (Coord, i32) {
    let cols = |l: i32| {
        map.iter()
            .filter_map(move |(&(ll, c), &ch)| if ll == l && ch != ' ' { Some(c) } else { None })
    };
    let lines = |c: i32| {
        map.iter()
            .filter_map(move |(&(l, cc), &ch)| if cc == c && ch != ' ' { Some(l) } else { None })
    };
    let min_c = |l: i32| cols(l).min().unwrap();
    let max_c = |l: i32| cols(l).max().unwrap();
    let min_l = |c: i32| lines(c).min().unwrap();
    let max_l = |c: i32| lines(c).max().unwrap();
    let mut pos = (0, min_c(0));
    let mut dir = 0;
    for mov in path.iter() {
        if let Move::Fwd(i) = mov {
            let dpos = DIRS[dir];
            let bnds = if dpos.0 == 0 {
                ((pos.0, pos.0), (min_c(pos.0), max_c(pos.0)))
            } else {
                ((min_l(pos.1), max_l(pos.1)), (pos.1, pos.1))
            };
            for _ in 0..*i {
                let new_pos = (
                    bnds.0 .0 + (pos.0 + dpos.0 - bnds.0 .0).rem_euclid(bnds.0 .1 - bnds.0 .0 + 1),
                    bnds.1 .0 + (pos.1 + dpos.1 - bnds.1 .0).rem_euclid(bnds.1 .1 - bnds.1 .0 + 1),
                );
                if let Some('#') = map.get(&new_pos) {
                    break;
                }
                pos = new_pos;
            }
        } else {
            dir = match mov {
                Move::R => (dir + 1) % 4,
                Move::L => (dir + 3) % 4,
                _ => unreachable!(),
            };
        }
    }
    (pos, dir as i32)
}

fn step1() {
    let (map, path) = read_input();
    let (pos, dir) = walk(&map, &path);
    let res = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + dir;
    println!("step1: {res}");
}

fn square_size(map: &Map) -> i32 {
    f32::sqrt(map.len() as f32 / 6.0).round() as i32
}

fn get_plane_face(map: &Map, coord: Coord) -> i32 {
    let size = square_size(map);
    let l = coord.0 / size;
    let c = coord.1 / size;
    (l * 4) + c
}

fn has_face(map: &Map, face_idx: i32) -> bool {
    let size = square_size(map);
    let (l, c) = face_idx_coord(face_idx);
    map.contains_key(&(l * size, c * size))
}

fn face_idx_coord(face_idx: i32) -> Coord {
    (face_idx / 4, face_idx % 4)
}

fn face_coord_idx(face_coord: Coord) -> i32 {
    assert!(face_coord.0 >= 0 && face_coord.0 < 4);
    assert!(face_coord.1 >= 0 && face_coord.1 < 4);
    face_coord.0 * 4 + face_coord.1
}

fn has_near(map: &Map, face_idx: i32, dir: usize) -> bool {
    let near_coord = vec_add(face_idx_coord(face_idx), DIRS[dir]);
    if near_coord.0 >= 4 || near_coord.1 >= 4 || near_coord.0 < 0 || near_coord.1 < 0 {
        false
    } else {
        has_face(map, face_coord_idx(near_coord))
    }
}

fn vec_scal(a: Coord, s: i32) -> Coord {
    (a.0 * s, a.1 * s)
}

fn vec_add(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn vec_dot(a: Coord, b: Coord) -> i32 {
    a.0 * b.0 + a.1 * b.1
}

fn is_unit(vec: Coord3D) -> bool {
    (vec.0 * vec.0 + vec.1 * vec.1 + vec.2 * vec.2) == 1
}

fn matrix_id() -> Matrix3D {
    ((1, 0, 0), (0, 1, 0), (0, 0, 1))
}

fn matrix_w(axis: Coord3D) -> Matrix3D {
    assert!(is_unit(axis));
    (
        (0, -axis.2, axis.1),
        (axis.2, 0, -axis.0),
        (-axis.1, axis.0, 0),
    )
}

fn mat_add(a: Matrix3D, b: Matrix3D) -> Matrix3D {
    (
        (a.0 .0 + b.0 .0, a.0 .1 + b.0 .1, a.0 .2 + b.0 .2),
        (a.1 .0 + b.1 .0, a.1 .1 + b.1 .1, a.1 .2 + b.1 .2),
        (a.2 .0 + b.2 .0, a.2 .1 + b.2 .1, a.2 .2 + b.2 .2),
    )
}

fn mat_mat(a: Matrix3D, b: Matrix3D) -> Matrix3D {
    (
        (
            a.0 .0 * b.0 .0 + a.0 .1 * b.1 .0 + a.0 .2 * b.2 .0,
            a.0 .0 * b.0 .1 + a.0 .1 * b.1 .1 + a.0 .2 * b.2 .1,
            a.0 .0 * b.0 .2 + a.0 .1 * b.1 .2 + a.0 .2 * b.2 .2,
        ),
        (
            a.1 .0 * b.0 .0 + a.1 .1 * b.1 .0 + a.1 .2 * b.2 .0,
            a.1 .0 * b.0 .1 + a.1 .1 * b.1 .1 + a.1 .2 * b.2 .1,
            a.1 .0 * b.0 .2 + a.1 .1 * b.1 .2 + a.1 .2 * b.2 .2,
        ),
        (
            a.2 .0 * b.0 .0 + a.2 .1 * b.1 .0 + a.2 .2 * b.2 .0,
            a.2 .0 * b.0 .1 + a.2 .1 * b.1 .1 + a.2 .2 * b.2 .1,
            a.2 .0 * b.0 .2 + a.2 .1 * b.1 .2 + a.2 .2 * b.2 .2,
        ),
    )
}

fn mat_scl(a: Matrix3D, s: i32) -> Matrix3D {
    (
        (a.0 .0 * s, a.0 .1 * s, a.0 .2 * s),
        (a.1 .0 * s, a.1 .1 * s, a.1 .2 * s),
        (a.2 .0 * s, a.2 .1 * s, a.2 .2 * s),
    )
}

fn mat_vec(a: Matrix3D, v: Coord3D) -> Coord3D {
    (
        (a.0 .0 * v.0 + a.0 .1 * v.1 + a.0 .2 * v.2),
        (a.1 .0 * v.0 + a.1 .1 * v.1 + a.1 .2 * v.2),
        (a.2 .0 * v.0 + a.2 .1 * v.1 + a.2 .2 * v.2),
    )
}

fn sin_n(n: i32) -> i32 {
    [0, 1, 0, -1][n.rem_euclid(4) as usize]
}

fn cos_n(n: i32) -> i32 {
    sin_n(n + 1)
}

fn mat_around(axis: Coord3D, n: i32) -> Matrix3D {
    let matrix_w = matrix_w(axis);
    mat_add(
        mat_add(matrix_id(), mat_scl(matrix_w, sin_n(n))),
        mat_scl(mat_mat(matrix_w, matrix_w), 1 - cos_n(n)),
    )
}

fn rotate_around(vec: Coord3D, axis: Coord3D, n: i32) -> Coord3D {
    let rot_matrix = mat_around(axis, n);
    mat_vec(rot_matrix, vec)
}

fn fold_square_walk(
    map: &Map,
    face_idx: i32,
    plane: Coord3D,
    axis_xy: (Coord3D, Coord3D),
    folded: &mut FaceRotMap,
) {
    if folded.contains_key(&face_idx) {
        return;
    }
    folded.insert(face_idx, (plane, axis_xy));
    let (axis_x, axis_y) = axis_xy;
    for (dir, axis, axis_rot, next_face_idx) in [
        (0, axis_y, 1, face_idx + 1),
        (1, axis_x, 1, face_idx + 4),
        (2, axis_y, -1, face_idx - 1),
        (3, axis_x, -1, face_idx - 4),
    ] {
        if has_near(map, face_idx, dir) {
            let new = rotate_around(plane, axis, axis_rot);
            let new_x = rotate_around(axis_x, axis, axis_rot);
            let new_y = rotate_around(axis_y, axis, axis_rot);
            fold_square_walk(map, next_face_idx, new, (new_x, new_y), folded);
        }
    }
}

fn fold_square(map: &Map) -> FaceRotMap {
    let min_c = |l: i32| {
        map.iter()
            .filter_map(|((ll, c), ch)| {
                if *ll == l && *ch != ' ' {
                    Some(*c)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    };
    let pos = (0, min_c(0));
    let face = get_plane_face(map, pos);
    let mut folded = FaceRotMap::new();
    fold_square_walk(map, face, (1, 1, 2), ((1, 0, 0), (0, 1, 0)), &mut folded);
    folded
}

fn get_face_plane(face_map: &FaceRotMap, facing: Coord3D) -> i32 {
    for (&idx, &(plane, _)) in face_map.iter() {
        if plane.0 == facing.0 && facing.0.abs() == 2
            || plane.1 == facing.1 && facing.1.abs() == 2
            || plane.2 == facing.2 && facing.2.abs() == 2
        {
            return idx;
        }
    }
    unreachable!();
}

fn get_rot_n(v1: Coord, v2: Coord) -> usize {
    let dot = vec_dot(v1, v2);
    [2, 1, 0][(dot.signum() + 1) as usize]
}

fn get_delta_rot_n(v1: Coord, v2: Coord) -> usize {
    let a = get_rot_n(v1, v2);
    let cross_z = v1.0 * v2.1 - v1.1 * v2.0;
    if cross_z != 0 {
        (a as i32 * cross_z.signum()).rem_euclid(4) as usize
    } else {
        a
    }
}

fn get_plane_proj(vec: Coord3D) -> Coord3D {
    if vec.0.abs() == 2 {
        rotate_around(vec, (0, 1, 0), -vec.0.signum())
    } else if vec.1.abs() == 2 {
        rotate_around(vec, (1, 0, 0), vec.1.signum())
    } else if vec.2.abs() == 2 {
        rotate_around(vec, (0, 1, 0), vec.2.signum() - 1)
    } else {
        unreachable!()
    }
}

fn get_proj_delta_rot_n(facing: Coord3D, rotated: Coord3D) -> usize {
    let p1 = get_plane_proj(facing);
    let p2 = get_plane_proj(rotated);
    assert!(p1.2 == p2.2);
    assert!(p1.2 == 2);
    get_delta_rot_n((p2.0, p2.1), (p1.0, p1.1))
}

fn to_graph(face_map: &FaceRotMap) -> FaceGraph {
    let mut graph = FaceGraph::new();
    for (&face_idx, &(plane, rot)) in face_map.iter() {
        for (rot_axis, rot_n, dir) in [(rot.1, 1, 0), (rot.0, 1, 1), (rot.1, -1, 2), (rot.0, -1, 3)]
        {
            let rotated_plane = rotate_around(plane, rot_axis, rot_n);
            let near_idx = get_face_plane(face_map, rotated_plane);
            let (near_plane, _) = *face_map.get(&near_idx).unwrap();
            let delta_rot_n = get_proj_delta_rot_n(near_plane, rotated_plane);
            graph.insert((face_idx, dir), (near_idx, delta_rot_n));
        }
    }
    graph
}

fn rotate_coord_right(map: &Map, coord: &Coord, rot: usize) -> Coord {
    let size = square_size(map);
    let mut new_coord = *coord;
    for _ in 0..rot {
        new_coord = (new_coord.1, size - 1 - new_coord.0);
    }
    new_coord
}

fn next_pos(
    map: &Map,
    graph: &FaceGraph,
    face_idx: i32,
    pos: Coord,
    dir: usize,
) -> (i32, Coord, usize) {
    let size = square_size(map);
    let vdir = DIRS[dir];
    let next = vec_add(pos, vdir);
    let bounded = ((next.0 + size) % size, (next.1 + size) % size);
    if next != bounded {
        let (new_face_idx, delta_rot_n) = *graph.get(&(face_idx, dir)).unwrap();
        let new_dir = (dir + delta_rot_n as usize) % 4;
        let new_coord = rotate_coord_right(map, &bounded, delta_rot_n);
        (new_face_idx, new_coord, new_dir)
    } else {
        (face_idx, next, dir)
    }
}

fn walk2(map: &Map, graph: &FaceGraph, path: &Path) -> (Coord, i32) {
    let min_c = |l: i32| {
        map.iter()
            .filter_map(
                |(&(ll, c), &ch)| {
                    if ll == l && ch != ' ' {
                        Some(c)
                    } else {
                        None
                    }
                },
            )
            .min()
            .unwrap()
    };
    let size = square_size(map);
    let mut pos = (0, min_c(0));
    let mut face_idx = get_plane_face(map, pos);
    let mut face_pos = (0, 0);
    let mut dir = 0;
    for mov in path.iter() {
        if let Move::Fwd(i) = mov {
            for _ in 0..*i {
                let (new_face_idx, new_face_pos, new_dir) =
                    next_pos(map, graph, face_idx, face_pos, dir);
                let new_pos = vec_add(new_face_pos, vec_scal(face_idx_coord(new_face_idx), size));
                if let Some('#') = map.get(&new_pos) {
                    break;
                }
                pos = new_pos;
                face_pos = new_face_pos;
                dir = new_dir;
                face_idx = new_face_idx;
            }
        } else {
            dir = match mov {
                Move::R => (dir + 1) % 4,
                Move::L => (dir + 3) % 4,
                _ => unreachable!(),
            };
        }
    }
    (pos, dir as i32)
}

fn step2() {
    let (map, path) = read_input();
    let folded = fold_square(&map);
    let graph = to_graph(&folded);
    let (pos, dir) = walk2(&map, &graph, &path);
    let res = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + dir;
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}

#[allow(dead_code)]
fn print_plane(plane: &Map) {
    let min_l = plane.keys().map(|c| c.0).min().unwrap();
    let max_l = plane.keys().map(|c| c.0).max().unwrap();
    let min_c = plane.keys().map(|c| c.1).min().unwrap();
    let max_c = plane.keys().map(|c| c.1).max().unwrap();
    for l in 0..=(max_l - min_l) {
        let line = (min_c..=max_c)
            .map(|c| {
                if let Some(ch) = plane.get(&(l + min_l, c)) {
                    ch.to_string()
                } else {
                    " ".to_string()
                }
            })
            .join("");
        println!("{}", line);
    }
}
