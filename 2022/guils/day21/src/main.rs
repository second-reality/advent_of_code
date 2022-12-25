use itertools::Itertools;
use std::collections::HashMap;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

enum Yell {
    Int(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}
type YellMap = HashMap<String, Yell>;

#[derive(Debug, Clone)]
enum Expr {
    Int(i64),
    X,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Equ(Box<Expr>, Box<Expr>),
    EquL(Box<Expr>, Box<Expr>),
}

fn read_input() -> YellMap {
    INPUT
        .trim()
        .split('\n')
        .map(|x| {
            let toks = x
                .replace(':', "")
                .split(' ')
                .map(str::to_string)
                .collect_vec();
            (
                toks[0].clone(),
                if toks.len() == 2 {
                    Yell::Int(toks[1].parse::<i64>().unwrap())
                } else {
                    let ops = (toks[1].clone(), toks[3].clone());
                    match &toks[2][..] {
                        "+" => Yell::Add(ops.0, ops.1),
                        "*" => Yell::Mul(ops.0, ops.1),
                        "-" => Yell::Sub(ops.0, ops.1),
                        "/" => Yell::Div(ops.0, ops.1),
                        _ => unreachable!(),
                    }
                },
            )
        })
        .collect()
}

fn compute(map: &YellMap, start: &String) -> i64 {
    let yell = map.get(start).unwrap();
    match yell {
        Yell::Int(x) => *x,
        Yell::Add(x, y) => compute(map, x) + compute(map, y),
        Yell::Mul(x, y) => compute(map, x) * compute(map, y),
        Yell::Sub(x, y) => compute(map, x) - compute(map, y),
        Yell::Div(x, y) => compute(map, x) / compute(map, y),
    }
}

fn step1() {
    let input = read_input();
    let res = compute(&input, &String::from("root"));
    println!("step1: {res}");
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn is_rat(expr: &Expr) -> Option<(i64, i64)> {
    if let Expr::Div(x, y) = expr {
        if let Expr::Int(i) = **x {
            if let Expr::Int(j) = **y {
                return Some((i, j));
            }
        }
    } else if let Expr::Int(i) = *expr {
        return Some((i, 1));
    }
    None
}

fn simplify(expr: Expr) -> Expr {
    if let Expr::Add(x, y) = expr {
        if let Some((a, b)) = is_rat(&x) {
            if let Some((c, d)) = is_rat(&y) {
                let l = lcm(b, d);
                if l.abs() == 1 {
                    return Expr::Int((a * l / b + c * l / d) / l);
                } else {
                    return Expr::Div(
                        Box::new(Expr::Int(a * l / b + c * l / d)),
                        Box::new(Expr::Int(l)),
                    );
                }
            }
        }
        if let Expr::Int(_) = *x {
            match *y {
                Expr::Add(t, u) => Expr::Add(t, Box::new(simplify(Expr::Add(u, x)))),
                Expr::Mul(_, _) => Expr::Add(y, x),
                Expr::Div(_, _) => Expr::Add(y, x),
                Expr::X => Expr::Add(y, x),
                _ => unreachable!(),
            }
        } else if let Expr::Int(_) = *y {
            simplify(Expr::Add(y, x))
        } else {
            Expr::Add(x, y)
        }
    } else if let Expr::Sub(x, y) = expr {
        simplify(Expr::Add(
            x,
            Box::new(simplify(Expr::Mul(y, Box::new(Expr::Int(-1))))),
        ))
    } else if let Expr::Mul(x, y) = expr {
        if let Expr::Int(i) = *x {
            let x_ = x.clone();
            match *y {
                Expr::Int(j) => Expr::Int(i * j),
                Expr::Add(t, u) => Expr::Add(
                    Box::new(simplify(Expr::Mul(t, x))),
                    Box::new(simplify(Expr::Mul(u, x_))),
                ),
                Expr::Mul(t, u) => Expr::Mul(t, Box::new(simplify(Expr::Mul(u, x)))),
                Expr::Div(t, u) => simplify(Expr::Div(Box::new(simplify(Expr::Mul(x, t))), u)),
                Expr::X => Expr::Mul(y, x),
                _ => unreachable!(),
            }
        } else if let Expr::Int(_) = *y {
            simplify(Expr::Mul(y, x))
        } else {
            Expr::Mul(x, y)
        }
    } else if let Expr::Div(x, y) = expr {
        if let Expr::Int(j) = *y {
            if let Expr::Int(i) = *x {
                if i % j == 0 {
                    Expr::Int(i / j)
                } else {
                    let g = gcd(i, j);
                    Expr::Div(Box::new(Expr::Int(i / g)), Box::new(Expr::Int(j / g)))
                }
            } else {
                let y_ = y.clone();
                match *x {
                    Expr::Add(t, u) => simplify(Expr::Add(
                        Box::new(simplify(Expr::Div(t, y))),
                        Box::new(simplify(Expr::Div(u, y_))),
                    )),
                    Expr::Mul(t, u) => simplify(Expr::Mul(t, Box::new(simplify(Expr::Div(u, y))))),
                    Expr::Div(t, u) => simplify(Expr::Div(t, Box::new(simplify(Expr::Mul(u, y))))),
                    _ => unreachable!(),
                }
            }
        } else if let Expr::Div(t, u) = *y {
            simplify(Expr::Div(Box::new(simplify(Expr::Mul(x, u))), t))
        } else {
            Expr::Div(x, y)
        }
    } else if let Expr::Equ(x, y) = expr {
        if let Expr::Int(_i) = *x {
            simplify(Expr::EquL(x, y))
        } else if let Expr::Int(_) = *y {
            simplify(Expr::EquL(y, x))
        } else {
            unreachable!();
        }
    } else if let Expr::EquL(x, y) = expr {
        match *y {
            Expr::Div(t, u) => simplify(Expr::EquL(Box::new(simplify(Expr::Mul(x, u))), t)),
            Expr::Add(t, u) => simplify(Expr::EquL(Box::new(simplify(Expr::Sub(x, u))), t)),
            Expr::Mul(t, u) => simplify(Expr::EquL(Box::new(simplify(Expr::Div(x, u))), t)),
            Expr::X => Expr::Equ(Box::new(Expr::X), x),
            _ => unreachable!(),
        }
    } else {
        expr
    }
}

fn solve(expr: Expr) -> i64 {
    if let Expr::Equ(_, y) = expr {
        if let Expr::Int(i) = *y {
            return i;
        }
    }
    unreachable!();
}

fn build_expr(map: &YellMap, start: &String) -> Expr {
    let yell = map.get(start).unwrap();
    if start.eq("root") {
        if let Yell::Add(x, y) = yell {
            simplify(Expr::Equ(
                Box::new(simplify(build_expr(map, x))),
                Box::new(simplify(build_expr(map, y))),
            ))
        } else {
            unreachable!()
        }
    } else if start.eq("humn") {
        Expr::X
    } else {
        match yell {
            Yell::Int(x) => Expr::Int(*x),
            Yell::Add(x, y) => simplify(Expr::Add(
                Box::new(build_expr(map, x)),
                Box::new(build_expr(map, y)),
            )),
            Yell::Mul(x, y) => simplify(Expr::Mul(
                Box::new(build_expr(map, x)),
                Box::new(build_expr(map, y)),
            )),
            Yell::Sub(x, y) => simplify(Expr::Sub(
                Box::new(build_expr(map, x)),
                Box::new(build_expr(map, y)),
            )),
            Yell::Div(x, y) => simplify(Expr::Div(
                Box::new(build_expr(map, x)),
                Box::new(build_expr(map, y)),
            )),
        }
    }
}

fn step2() {
    let input = read_input();
    let expr = build_expr(&input, &String::from("root"));
    let res = solve(expr);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
