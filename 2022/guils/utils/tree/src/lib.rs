use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, Clone, Default)]
struct OrdNode {
    count: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    bf: i32,
}

#[derive(Debug, Clone)]
pub struct OrdTree<T> {
    root: Option<usize>,
    tree: Vec<OrdNode>,
    elts: Vec<T>,
}

impl<T> Default for OrdTree<T> {
    fn default() -> Self {
        OrdTree {
            root: None,
            tree: Vec::<OrdNode>::new(),
            elts: Vec::<T>::new(),
        }
    }
}

impl<T> OrdTree<T> {
    pub fn new() -> OrdTree<T> {
        OrdTree::default()
    }

    pub fn len(&self) -> usize {
        self.root.map(|x| self.tree[x].count).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    #[allow(dead_code)]
    fn balance_height(&self, nid: usize, trap: bool, check_bf: bool) -> (bool, i32) {
        assert!(self.is_connected(nid));
        let (mut left_b, mut left_h, mut right_b, mut right_h) = (true, 0, true, 0);
        if let Some(left) = self.left_cell(nid) {
            (left_b, left_h) = self.balance_height(left, trap, check_bf);
        }
        if let Some(right) = self.right_cell(nid) {
            (right_b, right_h) = self.balance_height(right, trap, check_bf);
        }
        if trap {
            assert!(right_h - left_h == self.node(nid).bf);
        }
        let bf_ok = !check_bf || right_h - left_h == self.node(nid).bf;
        (
            left_b && right_b && (right_h - left_h).abs() <= 1 && bf_ok,
            cmp::max(right_h, left_h) + 1,
        )
    }

    #[allow(dead_code)]
    fn is_balanced(&self) -> bool {
        self.root
            .map(|x| self.balance_height(x, false, true).0)
            .unwrap_or(true)
    }

    #[allow(dead_code)]
    fn assert_balanced(&self) -> bool {
        self.root
            .map(|x| self.balance_height(x, true, true).0)
            .unwrap_or(true)
    }

    fn node(&self, nid: usize) -> &OrdNode {
        &self.tree[nid]
    }

    fn node_mut(&mut self, nid: usize) -> &mut OrdNode {
        &mut self.tree[nid]
    }

    fn parent_cell(&self, nid: usize) -> Option<usize> {
        self.tree[nid].parent
    }

    fn left_cell(&self, nid: usize) -> Option<usize> {
        self.tree[nid].left
    }

    fn right_cell(&self, nid: usize) -> Option<usize> {
        self.tree[nid].right
    }

    fn parent_dir_cell(&self, nid: usize) -> Option<(usize, i32)> {
        self.parent_cell(nid).map(|parent| {
            (
                parent,
                if self.left_cell(parent) == Some(nid) {
                    -1
                } else {
                    1
                },
            )
        })
    }

    fn left_count(&self, nid: usize) -> usize {
        self.left_cell(nid).map(|x| self.tree[x].count).unwrap_or(0)
    }

    fn right_count(&self, nid: usize) -> usize {
        self.right_cell(nid)
            .map(|x| self.tree[x].count)
            .unwrap_or(0)
    }

    pub fn first_cell(&self) -> Option<usize> {
        self.root.map(|x| self.sub_first(x))
    }

    pub fn last_cell(&self) -> Option<usize> {
        self.root.map(|x| self.sub_last(x))
    }

    fn sub_first(&self, nid: usize) -> usize {
        assert!(self.is_set(nid));
        self.left_cell(nid)
            .map(|x| self.sub_first(x))
            .unwrap_or(nid)
    }

    fn sub_last(&self, nid: usize) -> usize {
        assert!(self.is_set(nid));
        self.right_cell(nid)
            .map(|x| self.sub_last(x))
            .unwrap_or(nid)
    }

    fn is_set(&self, nid: usize) -> bool {
        self.tree[nid].count != 0
    }

    fn is_connected(&self, nid: usize) -> bool {
        self.tree[nid].parent.is_some() || self.root == Some(nid)
    }

    pub fn next_cell(&self, nid: usize) -> Option<usize> {
        assert!(self.is_set(nid));
        if let Some(right) = self.right_cell(nid) {
            return Some(self.sub_first(right));
        }
        let mut current = nid;
        while let Some((parent, dir)) = self.parent_dir_cell(current) {
            if dir < 0 {
                return Some(parent);
            }
            current = parent;
        }
        None
    }

    pub fn prev_cell(&self, nid: usize) -> Option<usize> {
        assert!(self.is_set(nid));
        if let Some(left) = self.left_cell(nid) {
            return Some(self.sub_last(left));
        }
        let mut current = nid;
        while let Some((parent, dir)) = self.parent_dir_cell(current) {
            if dir > 0 {
                return Some(parent);
            }
            current = parent;
        }
        None
    }

    pub fn get_cell(&self, idx: usize) -> Option<usize> {
        if idx >= self.len() {
            return None;
        }
        let mut current = self.root.unwrap();
        let mut current_idx = idx;
        while let Some(next) = {
            let left_count = self.left_count(current);
            match current_idx.cmp(&left_count) {
                Ordering::Less => self.left_cell(current),
                Ordering::Greater => {
                    current_idx -= left_count + 1;
                    self.right_cell(current)
                }
                _ => None,
            }
        } {
            current = next;
        }
        Some(current)
    }

    pub fn index_cell(&self, nid: usize) -> usize {
        let mut idx = self.left_count(nid);
        let mut current = nid;
        while self.parent_cell(current).is_some() {
            let (parent, dir) = self.parent_dir_cell(current).unwrap();
            if dir > 0 {
                idx += 1 + self.left_count(parent);
            }
            current = parent;
        }
        idx
    }

    pub fn get_val(&self, nid: usize) -> &T {
        &self.elts[nid]
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.get_cell(idx).map(|x| self.get_val(x))
    }

    pub fn remove_cell(&mut self, idx: usize) -> usize {
        assert!(idx < self.len());
        self.get_cell(idx)
            .map(|x| {
                self.remove_at_cell(x);
                x
            })
            .unwrap()
    }

    pub fn remove(&mut self, idx: usize) -> &T {
        let nid = self.remove_cell(idx);
        self.get_val(nid)
    }

    fn detach_parent(&mut self, nid: usize) -> Option<(usize, i32)> {
        assert!(self.is_set(nid));
        assert!(self.is_connected(nid));
        if let Some((parent, dir)) = self.parent_dir_cell(nid) {
            if dir < 0 {
                self.tree[parent].left = None;
            } else {
                self.tree[parent].right = None;
            }
            self.tree[nid].parent = None;
            self.tree[parent].count -= self.tree[nid].count;
            Some((parent, dir))
        } else {
            self.root = None;
            None
        }
    }

    fn attach_parent(&mut self, parent: Option<(usize, i32)>, nid: Option<usize>) {
        if let Some((parent, dir)) = parent {
            if dir < 0 {
                assert!(self.tree[parent].left.is_none());
                self.tree[parent].left = nid;
            } else {
                assert!(self.tree[parent].right.is_none());
                self.tree[parent].right = nid;
            }
            if let Some(nid) = nid {
                self.tree[parent].count += self.tree[nid].count;
                self.tree[nid].parent = Some(parent);
            }
            self.update_parent_count(parent);
        } else {
            self.root = nid;
        }
    }

    fn update_parent_count(&mut self, nid: usize) {
        assert!(self.is_set(nid));
        let mut current = nid;
        while let Some(parent) = self.parent_cell(current) {
            let count = 1 + self.left_count(parent) + self.right_count(parent);
            if count == self.tree[parent].count {
                break;
            }
            self.tree[parent].count = count;
            current = parent;
        }
    }

    fn attach_right(&mut self, nid: usize, right_nid: Option<usize>) {
        assert!(self.tree[nid].right.is_none());
        if let Some(right_nid) = right_nid {
            self.tree[right_nid].parent = Some(nid);
            self.tree[nid].right = Some(right_nid);
            self.tree[nid].count += self.tree[right_nid].count;
        }
        self.update_parent_count(nid);
    }

    fn attach_left(&mut self, nid: usize, left_nid: Option<usize>) {
        assert!(self.tree[nid].left.is_none());
        if let Some(left_nid) = left_nid {
            self.tree[left_nid].parent = Some(nid);
            self.tree[nid].left = Some(left_nid);
            self.tree[nid].count += self.tree[left_nid].count;
        }
        self.update_parent_count(nid);
    }

    fn detach_left(&mut self, nid: usize) -> Option<usize> {
        assert!(self.is_set(nid));
        let left_nid = self.tree[nid].left;
        if let Some(left_nid) = left_nid {
            self.tree[left_nid].parent = None;
            self.tree[nid].left = None;
            self.tree[nid].count -= self.tree[left_nid].count;
        }
        left_nid
    }

    fn detach_right(&mut self, nid: usize) -> Option<usize> {
        assert!(self.is_set(nid));
        let right_nid = self.tree[nid].right;
        if let Some(right_nid) = right_nid {
            self.tree[right_nid].parent = None;
            self.tree[nid].right = None;
            self.tree[nid].count -= self.tree[right_nid].count;
        }
        right_nid
    }

    fn insert_cell_root(&mut self, new_nid: usize) {
        assert!(!self.is_set(new_nid));
        assert!(self.root.is_none());
        self.root = Some(new_nid);
        self.tree[new_nid] = OrdNode {
            count: 1,
            ..OrdNode::default()
        }
    }

    pub fn insert_cell(&mut self, idx: usize, new_nid: usize) {
        if idx < self.len() {
            self.insert_at_cell(self.get_cell(idx), new_nid);
        } else if idx == 0 {
            self.insert_cell_root(new_nid);
        }
    }

    fn rotate_right_raw(&mut self, parent: usize, left_nid: usize) -> usize {
        let old_count = self.tree[parent].count;
        let prev_parent = self.detach_parent(parent);
        let _parent_dir = self.detach_parent(left_nid);
        if let Some(right) = self.right_cell(left_nid) {
            self.detach_right(left_nid);
            self.attach_left(parent, Some(right));
        }
        self.attach_right(left_nid, Some(parent));
        assert!(old_count == self.tree[left_nid].count);
        self.attach_parent(prev_parent, Some(left_nid));
        left_nid
    }

    fn rotate_right(&mut self, parent: usize, left_nid: usize) -> usize {
        let ret = self.rotate_right_raw(parent, left_nid);
        if self.node(left_nid).bf == 0 {
            self.node_mut(parent).bf = -1;
            self.node_mut(left_nid).bf = 1;
        } else {
            self.node_mut(parent).bf = 0;
            self.node_mut(left_nid).bf = 0;
        }
        ret
    }

    fn rotate_left_raw(&mut self, parent: usize, right_nid: usize) -> usize {
        let old_count = self.tree[parent].count;
        let prev_parent = self.detach_parent(parent);
        let _parent_dir = self.detach_parent(right_nid);
        if let Some(left) = self.left_cell(right_nid) {
            self.detach_left(right_nid);
            self.attach_right(parent, Some(left));
        }
        self.attach_left(right_nid, Some(parent));
        assert!(old_count == self.tree[right_nid].count);
        self.attach_parent(prev_parent, Some(right_nid));
        right_nid
    }

    fn rotate_left(&mut self, parent: usize, right_nid: usize) -> usize {
        let ret = self.rotate_left_raw(parent, right_nid);
        if self.node(right_nid).bf == 0 {
            self.node_mut(parent).bf = 1;
            self.node_mut(right_nid).bf = -1;
        } else {
            self.node_mut(parent).bf = 0;
            self.node_mut(right_nid).bf = 0;
        }
        ret
    }

    fn rotate_right_left(&mut self, parent: usize, right_nid: usize) -> usize {
        let old_count = self.tree[parent].count;
        let right_left_nid = self.left_cell(right_nid).unwrap();
        self.rotate_right_raw(right_nid, right_left_nid);
        self.rotate_left_raw(parent, right_left_nid);
        assert!(old_count == self.tree[right_left_nid].count);
        let bf = self.node(right_left_nid).bf;
        if bf == 0 {
            self.node_mut(parent).bf = 0;
            self.node_mut(right_nid).bf = 0;
        } else {
            if bf > 0 {
                self.node_mut(parent).bf = -1;
                self.node_mut(right_nid).bf = 0;
            } else {
                self.node_mut(parent).bf = 0;
                self.node_mut(right_nid).bf = 1;
            }
            self.node_mut(right_left_nid).bf = 0;
        }
        right_left_nid
    }

    fn rotate_left_right(&mut self, parent: usize, left_nid: usize) -> usize {
        let old_count = self.tree[parent].count;
        let left_right_nid = self.right_cell(left_nid).unwrap();
        self.rotate_left_raw(left_nid, left_right_nid);
        self.rotate_right_raw(parent, left_right_nid);
        assert!(old_count == self.tree[left_right_nid].count);
        let bf = self.node(left_right_nid).bf;
        if bf == 0 {
            self.node_mut(parent).bf = 0;
            self.node_mut(left_nid).bf = 0;
        } else {
            if bf < 0 {
                self.node_mut(parent).bf = 1;
                self.node_mut(left_nid).bf = 0;
            } else {
                self.node_mut(parent).bf = 0;
                self.node_mut(left_nid).bf = -1;
            }
            self.node_mut(left_right_nid).bf = 0;
        }

        left_right_nid
    }

    fn insert_at_cell_prev(&mut self, nid: Option<usize>, new_nid: usize) -> Option<(usize, i32)> {
        assert!(!self.is_set(new_nid));
        if let Some(nid) = nid {
            assert!(self.is_set(nid));
            self.tree[new_nid].count = 1;
            let parent_dir = if let Some(left) = self.left_cell(nid) {
                Some((self.sub_last(left), 1))
            } else {
                Some((nid, -1))
            };
            self.attach_parent(parent_dir, Some(new_nid));
            parent_dir
        } else {
            self.insert_cell_root(new_nid);
            None
        }
    }

    pub fn insert_at_cell(&mut self, nid: Option<usize>, new_nid: usize) {
        let parent_dir = self.insert_at_cell_prev(nid, new_nid);
        self.update_parent_bf_insert(new_nid, parent_dir);
    }

    fn update_parent_bf_insert(&mut self, new_nid: usize, parent_dir: Option<(usize, i32)>) {
        let mut current = new_nid;
        let mut parent_dir = parent_dir;
        while let Some((parent, dir)) = parent_dir {
            if dir < 0 {
                if self.node(parent).bf < 0 {
                    if self.node(current).bf > 0 {
                        self.rotate_left_right(parent, current);
                    } else {
                        self.rotate_right(parent, current);
                    }
                    break;
                } else {
                    self.node_mut(parent).bf -= 1;
                    if self.node(parent).bf == 0 {
                        break;
                    }
                }
            } else if dir > 0 {
                if self.node(parent).bf > 0 {
                    if self.node(current).bf < 0 {
                        self.rotate_right_left(parent, current);
                    } else {
                        self.rotate_left(parent, current);
                    }
                    break;
                } else {
                    self.node_mut(parent).bf += 1;
                    if self.node(parent).bf == 0 {
                        break;
                    }
                }
            }
            current = parent;
            parent_dir = self.parent_dir_cell(current);
        }
    }

    fn remove_at_cell_prev(&mut self, nid: usize) -> Option<(usize, i32)> {
        assert!(self.is_connected(nid));
        let parent_dir = self.detach_parent(nid);
        let nid_left = self.detach_left(nid);
        let nid_right = self.detach_right(nid);
        let nid_bf = self.node(nid).bf;
        *self.node_mut(nid) = OrdNode::default();
        if nid_left.is_none() && nid_right.is_none() {
            self.attach_parent(parent_dir, None);
            parent_dir
        } else if nid_left.is_none() {
            self.attach_parent(parent_dir, nid_right);
            parent_dir
        } else if nid_right.is_none() {
            self.attach_parent(parent_dir, nid_left);
            parent_dir
        } else {
            let replace = self.sub_last(nid_left.unwrap());
            if Some(replace) == nid_left {
                self.attach_right(replace, nid_right);
                self.attach_parent(parent_dir, Some(replace));
                self.node_mut(replace).bf = nid_bf;
                Some((replace, -1))
            } else {
                let parent_replace = self.detach_parent(replace);
                let replace_left = self.detach_left(replace);
                self.attach_right(parent_replace.unwrap().0, replace_left);
                self.attach_left(replace, nid_left);
                self.attach_right(replace, nid_right);
                self.attach_parent(parent_dir, Some(replace));
                self.node_mut(replace).bf = nid_bf;
                parent_replace
            }
        }
    }

    fn update_parent_bf_remove(&mut self, parent_dir: Option<(usize, i32)>) {
        let mut parent_dir = parent_dir;
        while let Some((parent, dir)) = parent_dir {
            let current;
            if dir < 0 {
                if self.node(parent).bf > 0 {
                    let right = self.right_cell(parent).unwrap();
                    let bf = self.node(right).bf;
                    if bf < 0 {
                        current = self.rotate_right_left(parent, right);
                    } else {
                        current = self.rotate_left(parent, right);
                    }
                    if bf == 0 {
                        break;
                    }
                } else {
                    self.node_mut(parent).bf += 1;
                    if self.node(parent).bf == 1 {
                        break;
                    }
                    current = parent;
                }
            } else if dir > 0 {
                if self.node(parent).bf < 0 {
                    let left = self.left_cell(parent).unwrap();
                    let bf = self.node(left).bf;
                    if bf > 0 {
                        current = self.rotate_left_right(parent, left);
                    } else {
                        current = self.rotate_right(parent, left);
                    }
                    if bf == 0 {
                        break;
                    }
                } else {
                    self.node_mut(parent).bf -= 1;
                    if self.node(parent).bf == -1 {
                        break;
                    }
                    current = parent;
                }
            } else {
                unreachable!();
            }
            parent_dir = self.parent_dir_cell(current);
        }
    }

    pub fn remove_at_cell(&mut self, nid: usize) {
        let parent_dir = self.remove_at_cell_prev(nid);
        self.update_parent_bf_remove(parent_dir);
    }
}

impl<T: std::fmt::Debug> std::iter::FromIterator<T> for OrdTree<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut tree = OrdTree::<T>::new();
        tree.root = None;
        tree.elts = iter.into_iter().collect();
        let len = tree.elts.len();
        tree.tree = (0..len).map(|_| OrdNode::default()).collect();
        for cell in (0..len).rev() {
            tree.insert_cell(0, cell);
        }
        tree
    }
}

pub struct OrdTreeIterator<'a, T> {
    onto: &'a OrdTree<T>,
    next_cell: Option<usize>,
}

impl<T> OrdTree<T> {
    fn iter(&self) -> OrdTreeIterator<'_, T> {
        OrdTreeIterator {
            onto: self,
            next_cell: self.first_cell(),
        }
    }
}

impl<'a, T> Iterator for OrdTreeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cell) = self.next_cell {
            self.next_cell = self.onto.next_cell(cell);
            Some(&self.onto.elts[cell])
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a OrdTree<T> {
    type Item = &'a T;
    type IntoIter = OrdTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: std::marker::Copy> OrdTree<T> {
    pub fn to_vec(self) -> Vec<T> {
        self.into_iter().copied().collect()
    }

    pub fn as_vec(&self) -> Vec<T> {
        self.into_iter().copied().collect()
    }
}

impl<T: std::fmt::Debug> OrdTree<T> {
    #[allow(dead_code)]
    fn dump_cell(&self, nid: usize) {
        let node = &self.tree[nid];
        let left_str = if let Some(left) = self.left_cell(nid) {
            let node = &self.tree[left];
            format!("left {}({})[{}]", left, node.count, node.bf)
        } else {
            "_".to_string()
        };
        let right_str = if let Some(right) = self.right_cell(nid) {
            let node = &self.tree[right];
            format!("right {}({})[{}]", right, node.count, node.bf)
        } else {
            "_".to_string()
        };
        println!(
            "node {}({})[{}] = {:?}: {} {}",
            nid,
            node.count,
            node.bf,
            self.get_val(nid),
            left_str,
            right_str
        );
    }

    #[allow(dead_code)]
    fn dump_tree(&self) {
        let mut stack = Vec::<Option<usize>>::new();
        stack.push(self.root);
        while let Some(cell) = stack.pop() {
            if let Some(nid) = cell {
                self.dump_cell(nid);
                stack.push(self.left_cell(nid));
                stack.push(self.right_cell(nid));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord_tree_to_from_vec() {
        let vec = vec![1, 2, -3, 3, -2, 0, 4];
        let tree = vec.iter().copied().collect::<OrdTree<i64>>();
        let vec_tree = tree.as_vec();
        if vec != vec_tree {
            tree.dump_tree();
            assert_eq!(vec, vec_tree);
        }
        if !tree.is_balanced() {
            tree.dump_tree();
            assert!(tree.is_balanced());
        }
    }

    #[test]
    fn ord_tree_get_cell() {
        let vec = vec![1, 2, -3, 3, -2, 0, 4];
        let tree = vec.iter().copied().collect::<OrdTree<i64>>();
        for i in 0..vec.len() {
            if tree.get_cell(i).is_none() || tree.get_cell(i).unwrap() != i {
                tree.dump_tree();
                assert!(tree.get_cell(i).is_some());
                assert_eq!(tree.get_cell(i).unwrap(), i);
            }
        }
    }

    #[test]
    fn ord_tree_get_val() {
        let vec = vec![1, 2, -3, 3, -2, 0, 4];
        let tree = vec.iter().copied().collect::<OrdTree<i64>>();
        for (i, v) in vec.iter().enumerate() {
            if tree.get(i).unwrap() != v {
                tree.dump_tree();
                assert_eq!(tree.get(i).unwrap(), v);
            }
        }
    }

    #[test]
    fn ord_tree_remove() {
        let vec = vec![1, 2, -3, 3, -2, 0, 4];
        let mut tree = vec.iter().copied().collect::<OrdTree<i64>>();
        for (rem, v) in [(2, -3), (5, 4), (3, -2), (0, 1), (1, 3), (1, 0), (0, 2)] {
            let removed = *tree.remove(rem);
            if removed != v {
                tree.dump_tree();
                assert_eq!(removed, v);
            }
            if !tree.is_balanced() {
                tree.dump_tree();
                assert!(tree.is_balanced());
            }
        }
    }

    #[test]
    fn ord_tree_move_elts() {
        let mut vec = vec![1, 2, -3, 3, -2, 0, 4];
        let mut tree = vec.iter().copied().collect::<OrdTree<i64>>();
        for (rem, ins) in [(0, 1), (0, 2), (1, 4), (2, 5), (2, 0), (4, 4), (6, 4)] {
            let elt = vec.remove(rem as usize);
            let cell = tree.remove_cell(rem as usize);
            if !tree.is_balanced() {
                tree.dump_tree();
                assert!(tree.is_balanced());
            }
            if vec != tree.as_vec() {
                tree.dump_tree();
                assert_eq!(vec, tree.as_vec());
            }
            vec.insert(ins as usize, elt);
            tree.insert_cell(ins as usize, cell);
            if !tree.is_balanced() {
                tree.dump_tree();
                assert!(tree.is_balanced());
            }
            if vec != tree.as_vec() {
                tree.dump_tree();
                assert_eq!(vec, tree.as_vec());
            }
        }
    }

    fn ord_tree_remove_insert_n(input: &[i64], rounds: usize) -> OrdTree<i64> {
        let index_positions =
            |positions: &[usize], idx: usize| positions.iter().position(|&x| x == idx).unwrap();
        let positions_val = |positions: &[usize], vec: &[i64]| {
            positions.iter().map(|&x| vec[x]).collect::<Vec<i64>>()
        };
        let mut positions = (0..input.len()).collect::<Vec<usize>>();
        let mut tree = input.iter().copied().collect::<OrdTree<i64>>();
        let len = input.len() as i64;
        for _r in 0..rounds {
            for (idx, val) in input.iter().enumerate() {
                let pos = tree.index_cell(idx);
                let v_pos = index_positions(&positions, idx);
                assert!(pos == v_pos);
                let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
                positions.remove(pos);
                let tree_before_remove = tree.clone();
                let parent_dir = tree.remove_at_cell_prev(idx);
                let tree_before_remove_balance = tree.clone();
                tree.update_parent_bf_remove(parent_dir);
                if !tree.is_balanced() {
                    println!("After remove {idx} at pos {pos} (round {_r})");
                    println!("Tree before remove:");
                    tree_before_remove.dump_tree();
                    println!("Tree before remove balance (parent dir: {:?}):", parent_dir);
                    tree_before_remove_balance.dump_tree();
                    println!("Tree after remove:");
                    tree.dump_tree();
                    assert!(tree.is_balanced());
                }
                if positions_val(&positions, &input) != tree.as_vec() {
                    println!("After remove {idx} at pos {pos}");
                    println!("Tree before remove:");
                    tree_before_remove.dump_tree();
                    println!("Tree after remove:");
                    tree.dump_tree();
                    assert_eq!(positions_val(&positions, &input), tree.as_vec());
                }
                positions.insert(target_pos, idx);
                let tree_before = tree.clone();
                tree.insert_cell(target_pos, idx);
                if !tree.is_balanced() {
                    println!("After insert {idx} at target pos {target_pos}");
                    tree.dump_tree();
                    assert!(tree.is_balanced());
                }
                if positions_val(&positions, &input) != tree.as_vec() {
                    println!("After insert {idx} at target pos {target_pos}");
                    println!("Tree before remove {idx} at pos {pos}:");
                    tree_before_remove.dump_tree();
                    println!("Tree before insert:");
                    tree_before.dump_tree();
                    println!("After insert:");
                    tree.dump_tree();
                    assert_eq!(positions_val(&positions, &input), tree.as_vec());
                }
            }
        }
        tree
    }

    #[test]
    fn ord_tree_remove_insert() {
        let vec = vec![1, 2, -3, 3, -2, 0, 4];
        let tree = ord_tree_remove_insert_n(&vec, 1);
        let ref_vec = vec![-2, 1, 2, -3, 4, 0, 3];
        let res_vec = tree.to_vec();
        assert_eq!(ref_vec, res_vec);
    }

    #[test]
    fn ord_tree_remove_insert_10() {
        let key = 811589153;
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let vec = input.iter().map(|x| x * key as i64).collect::<Vec<i64>>();
        ord_tree_remove_insert_n(&vec, 10);
    }

    #[test]
    fn ord_tree_remove_rotate_left_bug() {
        let vec = vec![2, 0, 2, 3, 0];
        ord_tree_remove_insert_n(&vec, 10);
    }

    #[test]
    fn ord_tree_remove_insert_data() {
        let key = 811589153;
        let rounds = 10;
        let data = include_str!("../data/input.txt");
        let mut tree = data
            .trim()
            .split('\n')
            .map(|x| x.parse::<i64>().unwrap() * key)
            .collect::<OrdTree<i64>>();
        let len = tree.len() as i64;
        for _r in 0..rounds {
            for (idx, val) in tree.clone().iter().enumerate() {
                let pos = tree.index_cell(idx);
                let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
                tree.remove_at_cell(idx);
                tree.insert_cell(target_pos, idx);
            }
        }
        let pos_0 = tree.iter().take_while(|&x| *x != 0).count();
        assert_eq!(pos_0, 3697);
    }
}
