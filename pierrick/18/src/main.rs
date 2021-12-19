#[derive(Clone, Copy, PartialEq)]
struct NodeId(usize);

#[derive(Clone, Copy)]
enum Node {
    Value(u8),
    Parent(NodeId, NodeId),
}

struct Tree {
    nodes: Vec<Node>,
}

enum Side {
    Left,
    Right,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: vec![] }
    }

    fn create_node(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        NodeId(self.nodes.len() - 1)
    }

    fn create_node_from_str(&mut self, s: &str) -> NodeId {
        let s = s.trim();
        let mut chars: Vec<char> = s.chars().rev().collect();
        self.parse_node(&mut chars)
    }

    fn get_node(&self, id: NodeId) -> Node {
        *self.nodes.get(id.0).unwrap()
    }

    fn get_node_value(&self, id: NodeId) -> usize {
        let node = self.get_node(id);
        match node {
            Node::Value(value) => value as usize,
            _ => unreachable!(),
        }
    }

    fn get_node_mut(&mut self, id: NodeId) -> &mut Node {
        self.nodes.get_mut(id.0).unwrap()
    }

    fn parse_node(&mut self, chars: &mut Vec<char>) -> NodeId {
        let next = chars.pop();
        let node = match next {
            Some('[') => {
                let left = self.parse_node(chars);
                let separator = chars.pop();
                assert_eq!(Some(','), separator);
                let right = self.parse_node(chars);
                let end = chars.pop();
                assert_eq!(end, Some(']'));
                Node::Parent(left, right)
            }
            Some(c) if c.is_digit(10) => Node::Value(c.to_digit(10).unwrap() as u8),
            _ => unreachable!(),
        };

        self.create_node(node)
    }

    fn to_string(&self, root: NodeId) -> String {
        match self.get_node(root) {
            Node::Value(val) => val.to_string(),
            Node::Parent(left, right) => {
                format!("[{},{}]", self.to_string(left), self.to_string(right))
            }
        }
    }

    fn add_nodes(&mut self, left: NodeId, right: NodeId) -> NodeId {
        self.create_node(Node::Parent(left, right))
    }

    fn explode(&mut self, root: NodeId) -> NodeId {
        self.explode_current_depth(root, None, None, 0)
    }

    fn add_to_node(&mut self, other: Option<NodeId>, side: Side, quantity: usize) {
        match other {
            None => (),
            Some(id) => {
                let other = self.get_node(id);
                match other {
                    Node::Value(val) => *self.get_node_mut(id) = Node::Value(val + quantity as u8),
                    Node::Parent(left, right) => {
                        match side {
                            Side::Left => self.add_to_node(Some(left), Side::Left, quantity),
                            Side::Right => self.add_to_node(Some(right), Side::Right, quantity),
                        };
                    }
                }
            }
        }
    }

    fn explode_current_depth(
        &mut self,
        root: NodeId,
        on_left: Option<NodeId>,
        on_right: Option<NodeId>,
        depth: usize,
    ) -> NodeId {
        let no_explode = root;
        match self.get_node(root) {
            Node::Value(_) => no_explode,
            Node::Parent(old_left, old_right) => {
                assert!(depth <= 4);
                if depth == 4 {
                    self.add_to_node(on_left, Side::Right, self.get_node_value(old_left));
                    self.add_to_node(on_right, Side::Left, self.get_node_value(old_right));
                    return self.create_node(Node::Value(0));
                }

                // recursion
                let exploded_left =
                    self.explode_current_depth(old_left, on_left, Some(old_right), depth + 1);
                if exploded_left != old_left {
                    return self.create_node(Node::Parent(exploded_left, old_right));
                }

                let exploded_right =
                    self.explode_current_depth(old_right, Some(old_left), on_right, depth + 1);
                if exploded_right != old_right {
                    return self.create_node(Node::Parent(old_left, exploded_right));
                }

                no_explode
            }
        }
    }

    fn split(&mut self, root: NodeId) -> NodeId {
        let no_split = root;
        match self.get_node(root) {
            Node::Value(val) => {
                if val < 10 {
                    no_split
                } else {
                    let left = self.create_node(Node::Value(val / 2));
                    let right = self.create_node(Node::Value((val / 2) + val % 2));
                    self.create_node(Node::Parent(left, right))
                }
            }
            Node::Parent(left, right) => {
                let split_left = self.split(left);
                if left != split_left {
                    return self.create_node(Node::Parent(split_left, right));
                }

                let split_right = self.split(right);
                if right != split_right {
                    return self.create_node(Node::Parent(left, split_right));
                }

                no_split
            }
        }
    }

    fn reduce(&mut self, root: NodeId) -> NodeId {
        let new_exploded = self.explode(root);
        if new_exploded != root {
            return self.reduce(new_exploded);
        }

        let new_split = self.split(root);
        if new_split != root {
            return self.reduce(new_split);
        }

        root
    }

    fn add_and_reduce(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let add = self.add_nodes(left, right);
        self.reduce(add)
    }

    fn magnitude(&self, root: NodeId) -> usize {
        match self.get_node(root) {
            Node::Value(value) => value as usize,
            Node::Parent(left, right) => 3 * self.magnitude(left) + 2 * self.magnitude(right),
        }
    }
}

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[test]
fn parse() {
    fn one(s: &str) {
        let mut tree = Tree::new();
        let node = tree.create_node_from_str(s);
        assert_eq!(tree.to_string(node), s);
    }

    one("[1,2]");
    one("[[1,2],3]");
    one("[9,[8,7]]");
    one("[[1,9],[8,5]]");
    one("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    one("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
    one("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    INPUT.lines().for_each(one);
}

#[test]
fn add() {
    fn one(expected: &str, left: &str, right: &str) {
        let mut tree = Tree::new();
        let left = tree.create_node_from_str(left);
        let right = tree.create_node_from_str(right);
        let res = tree.add_nodes(left, right);
        assert_eq!(expected, tree.to_string(res));
    }

    one("[[1,2],[[3,4],5]]", "[1,2]", "[[3,4],5]");
}

#[test]
fn explode() {
    fn one(expected: &str, orig: &str) {
        let mut tree = Tree::new();
        let node = tree.create_node_from_str(orig);
        let exploded = tree.explode(node);
        assert_eq!(expected, tree.to_string(exploded));
    }

    one("[[[[0,9],2],3],4]", "[[[[[9,8],1],2],3],4]");
    one("[7,[6,[5,[7,0]]]]", "[7,[6,[5,[4,[3,2]]]]]");
    one("[[6,[5,[7,0]]],3]", "[[6,[5,[4,[3,2]]]],1]");
    one(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    );
    one(
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );
}

#[test]
fn add_and_reduce() {
    fn one(expected: &str, left: &str, right: &str) {
        let mut tree = Tree::new();
        let left = tree.create_node_from_str(left);
        let right = tree.create_node_from_str(right);
        let res = tree.add_and_reduce(left, right);
        assert_eq!(expected, tree.to_string(res));
    }

    one(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
    );

    one(
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    );
}

#[test]
fn magnitude() {
    fn one(expected: usize, s: &str) {
        let mut tree = Tree::new();
        let node = tree.create_node_from_str(s);
        assert_eq!(expected, tree.magnitude(node));
    }

    one(143, "[[1,2],[[3,4],5]]");
    one(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    one(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    one(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    one(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    one(
        3488,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    );
}

fn part1(s: &str) -> (String, usize) {
    let mut tree = Tree::new();
    let expressions: Vec<NodeId> = s
        .lines()
        .map(|row| tree.create_node_from_str(row))
        .collect();
    let total = expressions
        .iter()
        .skip(1)
        .fold(expressions[0], |acc, node| tree.add_and_reduce(acc, *node));
    (tree.to_string(total), tree.magnitude(total))
}

fn main() {
    assert_eq!(4140, part1(EXAMPLE).1);
    println!("{}", part1(INPUT).1);
}
