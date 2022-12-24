use std::collections::HashMap;
#[derive(Clone)]
enum Expr {
    UnknownConst,
    Const(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

type SymTable = HashMap<String, Expr>;

impl Expr {
    fn eval(&self, symbols: &mut SymTable) -> Option<isize> {
        use Expr::*;
        match self {
            UnknownConst => None,
            Const(int) => Some(*int),
            Add(name1, name2) | Sub(name1, name2) | Mul(name1, name2) | Div(name1, name2) => {
                // recursive call with memoization on variable with name1
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let int1 = subexpr1.eval(symbols);
                if let Some(i) = int1 {
                    symbols.insert(name1.clone(), Const(i));
                }
                // recursive call with memoization on variable with name2
                let subexpr2 = symbols.get(name2).unwrap().clone();
                let int2 = subexpr2.eval(symbols);
                if let Some(i) = int2 {
                    symbols.insert(name2.clone(), Const(i));
                }
                if let (None, _) | (_, None) = (int1, int2) {
                    None
                } else {
                    let int1 = int1.unwrap();
                    let int2 = int2.unwrap();
                    match self {
                        Add(..) => Some(int1 + int2),
                        Sub(..) => Some(int1 - int2),
                        Mul(..) => Some(int1 * int2),
                        Div(..) => Some(int1 / int2),
                        _ => panic!("WTF const, unknwn"),
                    }
                }
            }
        }
    }

    fn reverse_eval(&self, symbols: &mut SymTable, target: isize) -> isize {
        use Expr::*;
        match self {
            Const(_) => panic!("wtf const"),
            UnknownConst => target,
            Add(name1, name2) => {
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let subexpr2 = symbols.get(name2).unwrap().clone();
                if let (Const(i), other) | (other, Const(i)) = (subexpr1, subexpr2) {
                    other.reverse_eval(symbols, target - i)
                } else {
                    panic!("wtf add");
                }
            }
            Mul(name1, name2) => {
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let subexpr2 = symbols.get(name2).unwrap().clone();
                if let (Const(i), other) | (other, Const(i)) = (subexpr1, subexpr2) {
                    other.reverse_eval(symbols, target / i)
                } else {
                    panic!("wtf mul");
                }
            }
            Sub(name1, name2) => {
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let subexpr2 = symbols.get(name2).unwrap().clone();
                match (subexpr1, subexpr2) {
                    (Const(i), other) => other.reverse_eval(symbols, i - target),
                    (other, Const(i)) => other.reverse_eval(symbols, target + i),
                    _ => panic!("wtf sub"),
                }
            }
            Div(name1, name2) => {
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let subexpr2 = symbols.get(name2).unwrap().clone();
                match (subexpr1, subexpr2) {
                    (Const(i), other) => other.reverse_eval(symbols, i / target),
                    (other, Const(i)) => other.reverse_eval(symbols, target * i),
                    _ => panic!("wtf div"),
                }
            }
        }
    }
}
fn parse(input: String) -> SymTable {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut words = line.split(' ');
            let name = words.next().unwrap().replace(':', "");
            let first_id = words.next().unwrap();
            if let Ok(int) = first_id.parse::<isize>() {
                (name, Expr::Const(int))
            } else {
                let first_id = first_id.to_owned();
                let op = words.next().unwrap();
                let second_id = words.next().unwrap().to_owned();
                let expr = match op {
                    "+" => Expr::Add(first_id, second_id),
                    "-" => Expr::Sub(first_id, second_id),
                    "*" => Expr::Mul(first_id, second_id),
                    "/" => Expr::Div(first_id, second_id),
                    _ => panic!("WTF op"),
                };
                (name, expr)
            }
        })
        .collect()
}
pub fn part1(input: String) -> usize {
    let mut symbols = parse(input);
    let root = symbols.get("root").unwrap().clone();
    root.eval(&mut symbols).unwrap() as usize
}

pub fn part2(input: String) -> usize {
    let mut symbols = parse(input);
    symbols.insert("humn".to_owned(), Expr::UnknownConst);
    if let Expr::Add(dep1, dep2) = symbols.get("root").unwrap().clone() {
        let dep1 = symbols.get(&dep1).unwrap().clone();
        let res1 = dep1.eval(&mut symbols);
        let dep2 = symbols.get(&dep2).unwrap().clone();
        let res2 = dep2.eval(&mut symbols);
        match (res1, res2) {
            (Some(target), None) => {
                return dep2.reverse_eval(&mut symbols, target) as usize;
            }
            (None, Some(target)) => {
                return dep1.reverse_eval(&mut symbols, target) as usize;
            }
            _ => panic!("No dependancy is known"),
        }
    }
    panic!("No root");
}

pub const EXPECTED1: usize = 152;
pub const EXPECTED2: usize = 301;
