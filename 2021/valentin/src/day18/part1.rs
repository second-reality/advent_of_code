use std::{fmt, fs, ops};
use std::borrow::BorrowMut;

pub(crate) struct FishNumber {
    left: Option<Box<FishNumber>>,
    right: Option<Box<FishNumber>>,
    number: Option<u8>,
    is_left_child: bool,
    is_root: bool,
}

impl fmt::Debug for FishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Add<FishNumber> for FishNumber {
    type Output = FishNumber;

    fn add(self, _rhs: FishNumber) -> FishNumber {
        let this_pair = FishNumber {
            left: self.left,
            right: self.right,
            number: self.number,
            is_left_child: true,
            is_root: false,
        };

        let other_pair = FishNumber {
            left: _rhs.left,
            right: _rhs.right,
            number: _rhs.number,
            is_left_child: false,
            is_root: false,
        };

        FishNumber {
            left: Option::Some(Box::from(this_pair)),
            right: Option::Some(Box::from(other_pair)),
            number: Option::None,
            is_left_child: false,
            is_root: true,
        }
    }
}

impl FishNumber {
    pub(crate) fn reduce(&mut self) {
        let mut changes = true;
        while changes {
            changes = false;
            let mut path: Vec<bool> = Vec::new();
            let (to_add_left, to_add_right, path) = self.search_for_explosion(&mut path, 4);
            if path.len() > 0 {
                self.explode(&path, to_add_left, to_add_right);
                changes = true;
                continue
            }
            if self.search_for_split() {
                changes = true;
            }
        }

    }

    fn search_for_explosion(&self, path: &mut Vec<bool>, depth: u8) -> (u8, u8, Vec<bool>) {
        if depth == 0 && self.number.is_none() {
            // children should be terminal values
            let left_val = self.left.as_ref().unwrap().number.unwrap();
            let right_val = self.right.as_ref().unwrap().number.unwrap();
            // println!("explosion [{},{}]", left_val, right_val);
            path.push(self.is_left_child);
            return (left_val, right_val, path.clone());
        } else if depth > 0 && self.number.is_none() {
            if !self.is_root {
                path.push(self.is_left_child);
            }

            let res = self.left.as_ref().unwrap().search_for_explosion(path, depth - 1);
            if res.0 != u8::MAX {
                return res;
            }

            let res = self.right.as_ref().unwrap().search_for_explosion(path, depth - 1);
            if res.0 != u8::MAX {
                return res;
            }

            if !self.is_root {
                path.pop();
            }
        }
        (u8::MAX, u8::MAX, Vec::new())
    }

    fn move_on(&mut self, path: &Vec<bool>, imax: usize) -> &mut FishNumber {
        let mut cur = self;
        for i in 0..imax {
            cur = if path[i] {
                cur.left.as_mut().unwrap()
            } else {
                cur.right.as_mut().unwrap()
            }
        }
        cur
    }

    fn explode(&mut self, path: &Vec<bool>, to_add_left: u8, to_add_right: u8) {
        let mut cur = self.move_on(path, path.len());

        for i in (0..path.len()).rev() {
            if cur.is_left_child {
                cur = self.move_on(path, i);
            } else {
                cur = self.move_on(path, i).left.as_mut().unwrap();
                while cur.number.is_none() {
                    cur = cur.right.as_mut().unwrap();
                }

                // println!("left add {} to existing {}", to_add_left, cur.number.unwrap());
                cur.number = Option::Some(cur.number.unwrap() + to_add_left);
                break;
            }
        }
        cur = self.move_on(path, path.len());

        for i in (0..path.len()).rev() {

            if !cur.is_left_child {
                cur = self.move_on(path, i);
            } else {
                cur = self.move_on(path, i).right.as_mut().unwrap();
                while cur.number.is_none() {
                    cur = cur.left.as_mut().unwrap();
                }
                // println!("right add {} to existing {}", to_add_right, cur.number.unwrap());
                cur.number = Option::Some(cur.number.unwrap() + to_add_right);
                break;
            }
        }

        cur = self.move_on(path, path.len());
        // the pair explodes
        cur.left = Option::None;
        cur.right = Option::None;
        cur.number = Option::Some(0);
    }

    fn search_for_split(&mut self) -> bool {
        let mut stack: Vec<&mut FishNumber> = Vec::new();
        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if node.number.is_some() && node.number.unwrap() > 9 {
                let left_val = node.number.unwrap() / 2;
                let right_val = node.number.unwrap() - left_val;
                // println!("split {} -> [{},{}]",node.number.unwrap(), left_val, right_val);
                node.left = Option::Some(Box::from(FishNumber::from_num(left_val, true, false)));
                node.right = Option::Some(Box::from(FishNumber::from_num(right_val, false, false)));
                node.number = Option::None;
                return true;
            } else if node.number.is_none() {
                stack.push(node.right.as_mut().unwrap());
                stack.push(node.left.as_mut().unwrap());
            }
        }
        false
    }

    pub fn to_string(&self) -> String {
        if self.number.is_some() {
            format!("{}", self.number.unwrap())
        } else {
            let left = self.left.as_ref().unwrap().to_string();
            let right = self.right.as_ref().unwrap().to_string();
            format!("[{},{}]", left, right)
        }
    }

    fn from_num(number: u8, is_left_child: bool, is_root: bool) -> FishNumber {
        FishNumber {
            left: Option::None,
            right: Option::None,
            number: Option::Some(number),
            is_left_child,
            is_root,
        }
    }
    pub fn from(string: &str, is_left_child: bool, is_root: bool) -> FishNumber {
        if string.len() == 1 {
            FishNumber::from_num(string.parse::<u8>().unwrap(), is_left_child, is_root)
        } else {
            let mut depth = 0;
            let mut end_left = usize::MAX;
            for (index, c) in string.chars().enumerate() {
                match c {
                    '[' => {
                        depth += 1;
                    }
                    ']' => {
                        depth -= 1;
                    }
                    ',' => {
                        if depth == 1 {
                            end_left = index;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let left_num = FishNumber::from(&string[1..end_left], true, false);
            let right_num = FishNumber::from(&string[(end_left + 1)..(string.len() - 1)], false, false);
            FishNumber {
                left: Option::Some(Box::from(left_num)),
                right: Option::Some(Box::from(right_num)),
                number: Option::None,
                is_left_child,
                is_root,
            }
        }
    }

    pub fn magnitude(&self) -> usize {
        if self.number.is_some() {
            self.number.unwrap() as usize
        } else {
            let left = self.left.as_ref().unwrap();
            let right = self.right.as_ref().unwrap();
            3 * left.magnitude() + 2 * right.magnitude()
        }
    }
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day18/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut sum = FishNumber::from(lines[0], false, true);
    for i in 1..lines.len() {
        sum = sum + FishNumber::from(lines[i], false, true); ;
        sum.reduce();
        println!("{:?}", sum);
    }
    sum.magnitude()
}

fn _test() {
    let test = "[[[[4,3],4],4],[7,[[8,4],9]]]";
    let fish_num1 = FishNumber::from(test, false, true);
    let fish_num2 = FishNumber::from("[1,1]", false, true);
    let mut fish_num = fish_num1 + fish_num2;
    println!("fish number {:?}", fish_num);
    fish_num.reduce();
    println!("fish number {:?}", fish_num);
}