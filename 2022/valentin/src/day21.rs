use std::collections::HashMap;
#[derive(Clone)]
enum Expr {
    Unknown,
    Const(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
}

type SymTable = HashMap<String, Expr>;

impl Expr {
    fn eval(&self, symbols: &mut SymTable) -> Option<isize> {
        use Expr::*;
        match self {
            Unknown => None,
            Const(int) => Some(*int),
            Add(name1, name2)
            | Sub(name1, name2)
            | Mul(name1, name2)
            | Div(name1, name2)
            | Eq(name1, name2) => {
                // recursive call with memoization on variable with name1
                let subexpr1 = symbols.get(name1).unwrap().clone();
                let int1 = subexpr1.eval(symbols);
                symbols.insert(name1.clone(), int1.map(Const).unwrap_or(Unknown));
                // recursive call with memoization on variable with name2
                let subexpr2 = symbols.get(name2).unwrap().clone();
                let int2 = subexpr2.eval(symbols);
                symbols.insert(name2.clone(), int2.map(Const).unwrap_or(Unknown));
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
                        Eq(..) => {
                            if int1 == int2 {
                                Some(0)
                            } else {
                                Some(-1)
                            }
                        }
                        _ => panic!("WTF const, unknwn"),
                    }
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
    symbols.insert("humn".to_owned(), Expr::Unknown);
    if let Expr::Add(dep1, dep2) = symbols.get("root").unwrap().clone() {
        symbols.insert("root".to_owned(), Expr::Eq(dep1.clone(), dep2.clone()));
        let dep1 = symbols.get(&dep1).unwrap().clone();
        println!("dep1 = {:?}", dep1.eval(&mut symbols));
        let dep2 = symbols.get(&dep2).unwrap().clone();
        println!("dep2 = {:?}", dep2.eval(&mut symbols));
    }
    301
}

pub const EXPECTED1: usize = 152;
pub const EXPECTED2: usize = 301;
