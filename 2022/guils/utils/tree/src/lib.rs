use std::cmp;

#[derive(Debug, Clone)]
struct OrdNode {
    count: i32,
    parent: i32,
    left: i32,
    right: i32,
    bf: i32
}

impl Default for OrdNode {
    fn default() -> Self {
	OrdNode {
	    count: -1,
	    parent: -1,
	    left: -1,
	    right: -1,
	    bf: 0
	}
    }
}

#[derive(Debug, Clone)]
pub struct OrdTree<T> {
     root: i32,
     tree: Vec<OrdNode>,
     elts: Vec<T>,
}

impl<T> Default for OrdTree<T> {
    fn default() -> Self {
	OrdTree {
	    root: -1,
	    tree: Vec::<OrdNode>::new(),
	    elts: Vec::<T>::new()
	}
    }
}


impl<T> OrdTree<T> {
    pub fn new() -> OrdTree<T> {
	OrdTree::default()
    }

    pub fn len(&self) -> usize {
	if self.root == -1 {
	    0
	} else {
	    self.tree[self.root as usize].count as usize
	}
    }
    
    pub fn is_empty(&self) -> bool {
	self.root == -1
    }

    #[allow(dead_code)]
    fn balance_height(&self, nid: i32, trap: bool, check_bf: bool) -> (bool, i32) {
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
	(left_b && right_b && (right_h - left_h).abs() <= 1 && bf_ok, cmp::max(right_h, left_h) + 1)
    }

    #[allow(dead_code)]
    fn is_balanced(&self) -> bool {
	if self.root == -1 {
	    true
	} else {
	    self.balance_height(self.root, false, true).0
	}
    }

    #[allow(dead_code)]
    fn assert_balanced(&self) -> bool {
	if self.root == -1 {
	    true
	} else {
	    self.balance_height(self.root, true, true).0
	}
    }

    fn node(&self, nid: i32) -> &OrdNode {
	&self.tree[nid as usize]
    }

    fn node_mut(&mut self, nid: i32) -> &mut OrdNode {
	&mut self.tree[nid as usize]
    }
    
    fn parent_cell(&self, nid: i32) -> Option<i32> {
	let parent = self.tree[nid as usize].parent;
	if parent != -1 { Some(parent) } else { None }
    }

    fn parent_dir_cell(&self, nid: i32) -> Option<(i32, i32)> {
	let parent = self.get_parent_dir(nid);
	if parent.0 != - 1 {
	    Some(parent)
	} else {
	    None
	}
    }
    
    fn left_cell(&self, nid: i32) -> Option<i32> {
	let left = self.tree[nid as usize].left;
	if left != -1 { Some(left) } else { None }
    }

    fn right_cell(&self, nid: i32) -> Option<i32> {
	let right = self.tree[nid as usize].right;
	if right != -1 { Some(right) } else { None }
    }

    fn left_count(&self, nid: i32) -> i32 {
	if let Some(left) = self.left_cell(nid) {
	    self.tree[left as usize].count
	} else {
	    0
	}
    }

    fn right_count(&self, nid: i32) -> i32 {
	if let Some(right) = self.right_cell(nid) {
	    self.tree[right as usize].count
	} else {
	    0
	}
    }

    pub fn first_cell(&self) -> Option<i32> {
	if self.root == -1 {
	    None
	} else {
	    Some(self.sub_first(self.root))
	}
    }
    
    pub fn last_cell(&self) -> Option<i32> {
	if self.root == -1 {
	    None
	} else {
	    Some(self.sub_last(self.root))
	}
    }
    
    fn sub_first(&self, nid: i32) -> i32 {
	assert!(self.is_set(nid));
	if let Some(left) = self.left_cell(nid) {
	    self.sub_first(left)
	} else {
	    nid
	}
    }

    fn sub_last(&self, nid: i32) -> i32 {
	assert!(self.is_set(nid));
	if let Some(right) = self.right_cell(nid) {
	    self.sub_last(right)
	} else {
	    nid
	}
    }

    fn is_set(&self, nid: i32) -> bool {
	self.tree[nid as usize].count != -1
    }

    fn is_connected(&self, nid: i32) -> bool {
	self.tree[nid as usize].parent != -1 ||
	    self.root == nid
    }
    
    pub fn next_cell(&self, nid: i32) -> Option<i32> {
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

    pub fn prev_cell(&self, nid: i32) -> Option<i32> {
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

    pub fn get_cell(&self, idx: usize) -> i32 {
	assert!(idx < self.len());
	let mut current = self.root;
	assert!(current != -1);
	let mut current_idx = idx as i32;
	loop {
	    let left_count = self.left_count(current);
	    if current_idx == left_count {
		break;
	    } else if current_idx < left_count {
		current = self.left_cell(current).unwrap();
	    } else {
		current = self.right_cell(current).unwrap();
		current_idx -= left_count + 1;
	    }
	}
	current
    }

    pub fn index_cell(&self, nid: i32) -> usize {
	let mut idx = self.left_count(nid);
	let mut current = nid;
	while self.parent_cell(current).is_some() {
	    let (parent, dir) = self.get_parent_dir(current);
	    if dir > 0 {
		idx += 1 + self.left_count(parent);
	    }
	    current = parent;
	}
	idx as usize
    }

    pub fn get_val(&self, nid: i32) -> &T {
	assert!(nid != -1);
	&self.elts[nid as usize]
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
	if idx < self.len() {
	    Some(self.get_val(self.get_cell(idx)))
	} else {
	    None
	}
    }

    pub fn remove_at_cell_old(&mut self, nid: i32) {
	assert!(self.is_set(nid));
	// decrease parent count
	if self.parent_cell(nid).is_some() {
	    let mut parent = nid;
	    while self.parent_cell(parent).is_some() {
		parent = self.parent_cell(parent).unwrap();
		self.tree[parent as usize].count -= 1;
	    }
	}
	// attach left or right to parent or set root
	if let Some(left) = self.left_cell(nid) {
	    if let Some(parent) = self.parent_cell(nid) {
		self.tree[left as usize].parent = parent;
		if self.right_cell(parent).is_none() ||
		    self.right_cell(parent).unwrap() != nid {
			self.tree[parent as usize].left = left;
		    } else {
			self.tree[parent as usize].right = left;
		    }
	    } else {
		self.root = left;
		self.tree[left as usize].parent = -1;
	    }
	} else if let Some(right) = self.right_cell(nid) {
	    if let Some(parent) = self.parent_cell(nid) {
		self.tree[right as usize].parent = parent;
		if self.right_cell(parent).is_none() ||
		    self.right_cell(parent).unwrap() != nid {
			self.tree[parent as usize].left = right;
		    } else {
			self.tree[parent as usize].right = right;
		    }
	    } else {
		self.root = right;
		self.tree[right as usize].parent = -1;
	    }
	} else if let Some(parent) = self.parent_cell(nid) {
	    if self.right_cell(parent).is_none() ||
		self.right_cell(parent).unwrap() != nid {
		    self.tree[parent as usize].left = -1;
		} else {
		    self.tree[parent as usize].right = -1;
		}
	} else {
	    self.root = -1;
	}
	// attach right to rightmost left
	if let Some(right) = self.right_cell(nid) {
	    if let Some(left) = self.left_cell(nid) {
		let right_count = self.tree[right as usize].count;
		let last = left;
		self.tree[last as usize].count += right_count;
		while let Some(last) = self.right_cell(last) {
		    self.tree[last as usize].count += right_count;
		}
		self.tree[right as usize].parent = last;
		self.tree[last as usize].right = right;
	    }
	}
	
	self.tree[nid as usize] = OrdNode::default();
    }

    pub fn remove_cell(&mut self, idx: usize) -> i32 {
	assert!(idx < self.len());
	let nid = self.get_cell(idx);
	self.remove_at_cell(nid);
	nid
    }

    pub fn remove(&mut self, idx: usize) -> &T {
	let nid = self.remove_cell(idx);
	self.get_val(nid)
    }

    fn get_parent_dir(&self, nid: i32) -> (i32, i32) {
	if let Some(parent) = self.parent_cell(nid) {
	    if self.right_cell(parent).is_none() ||
		self.right_cell(parent).unwrap() != nid {
		    (parent, -1)
		} else {
		    (parent, 1)
		}
	} else {
	    (-1, 0)
	}
    }

    fn detach_parent(&mut self, nid: i32) -> (i32, i32) {
	assert!(self.is_set(nid));
	assert!(self.is_connected(nid));
	let (parent, dir) = self.get_parent_dir(nid);
	if dir != 0 {
	    if dir < 0 {
		self.tree[parent as usize].left = -1;
	    } else {
		self.tree[parent as usize].right = -1;

	    }
	    self.tree[nid as usize].parent = -1;
	    self.tree[parent as usize].count -= self.tree[nid as usize].count;
	} else {
	    self.root = -1;
	}
	(parent, dir)
    }

    fn attach_parent(&mut self, parent: (i32, i32), nid: i32) {
	if parent.0 != -1 {
	    if parent.1 < 0 {
		self.tree[parent.0 as usize].left = nid;
	    } else {
		self.tree[parent.0 as usize].right = nid;
	    }
	    if nid != -1 {
		self.tree[parent.0 as usize].count += self.tree[nid as usize].count;
		self.tree[nid as usize].parent = parent.0;
	    }
	    self.update_parent_count(parent.0);
	} else {
	    self.root = nid;
	}
    }

    fn update_parent_count(&mut self, nid: i32) {
	assert!(self.is_set(nid));
	let mut current = nid;
	while let Some(parent) = self.parent_cell(current)  {
	    let count = 1 + self.left_count(parent) + self.right_count(parent);
	    if  count == self.tree[parent as usize].count {
		break;
	    }
	    self.tree[parent as usize].count = count;
	    current = parent;
	}
    }
    
    fn attach_right(&mut self, nid: i32, right_nid: i32) {
	assert!(self.tree[nid as usize].right == -1);
	if right_nid != -1 {
	    self.tree[right_nid as usize].parent = nid;
	    self.tree[nid as usize].right = right_nid;
	    self.tree[nid as usize].count += self.tree[right_nid as usize].count;
	}
	self.update_parent_count(nid);
    }

    fn attach_left(&mut self, nid: i32, left_nid: i32) {
	assert!(self.tree[nid as usize].left == -1);
	if left_nid != -1 {
	    self.tree[left_nid as usize].parent = nid;
	    self.tree[nid as usize].left = left_nid;
	    self.tree[nid as usize].count += self.tree[left_nid as usize].count;
	}
	self.update_parent_count(nid);
    }

    fn detach_left(&mut self, nid: i32) -> i32 {
	assert!(self.is_set(nid));
	let left_nid = self.tree[nid as usize].left;
	if left_nid != -1 {
	    self.tree[left_nid as usize].parent = -1;
	    self.tree[nid as usize].left = -1;
	    self.tree[nid as usize].count -= self.tree[left_nid as usize].count;
	}
	left_nid
    }

    fn detach_right(&mut self, nid: i32) -> i32 {
	assert!(self.is_set(nid));
	let right_nid = self.tree[nid as usize].right;
	if right_nid != -1 {
	    self.tree[right_nid as usize].parent = -1;
	    self.tree[nid as usize].right = -1;
	    self.tree[nid as usize].count -= self.tree[right_nid as usize].count;
	}
	right_nid
    }

    fn insert_cell_root(&mut self, new_nid: i32) {
	assert!(!self.is_set(new_nid));
	assert!(self.root == -1);
	self.root = new_nid;
	self.tree[new_nid as usize] = OrdNode {
	    count: 1,
	    ..OrdNode::default()
	}
    }
    
    pub fn insert_cell(&mut self, idx: usize, new_nid: i32) {
	if idx < self.len() {
	    self.insert_at_cell(self.get_cell(idx), new_nid);
	} else if idx == 0 {
	    self.insert_cell_root(new_nid);
	}
    }

    fn rotate_right_raw(&mut self, parent: i32, left_nid: i32) -> i32 {
	let old_count = self.tree[parent as usize].count;
	let prev_parent = self.detach_parent(parent);
	let parent_dir = self.detach_parent(left_nid);
	assert!(parent_dir == (parent, -1));
	if let Some(right) = self.right_cell(left_nid) {
	    self.detach_right(left_nid);
	    self.attach_left(parent, right);
	}
	self.attach_right(left_nid, parent);
	assert!(old_count == self.tree[left_nid as usize].count);
	self.attach_parent(prev_parent, left_nid);
	left_nid
    }

    fn rotate_right(&mut self, parent: i32, left_nid: i32) -> i32 {
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

    fn rotate_left_raw(&mut self, parent: i32, right_nid: i32) -> i32 {
	let old_count = self.tree[parent as usize].count;
	let prev_parent = self.detach_parent(parent);
	let parent_dir = self.detach_parent(right_nid);
	assert!(parent_dir == (parent, 1));
	if let Some(left) = self.left_cell(right_nid) {
	    self.detach_left(right_nid);
	    self.attach_right(parent, left);
	}
	self.attach_left(right_nid, parent);
	assert!(old_count == self.tree[right_nid as usize].count);
	self.attach_parent(prev_parent, right_nid);
	right_nid
    }

    fn rotate_left(&mut self, parent: i32, right_nid: i32) -> i32 {
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

    fn rotate_right_left(&mut self, parent: i32, right_nid: i32) -> i32 {
	let old_count = self.tree[parent as usize].count;
	let right_left_nid = self.left_cell(right_nid).unwrap();
	self.rotate_right_raw(right_nid, right_left_nid);
	self.rotate_left_raw(parent, right_left_nid);
	assert!(old_count == self.tree[right_left_nid as usize].count);
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

    fn rotate_left_right(&mut self, parent: i32, left_nid: i32) -> i32 {
	let old_count = self.tree[parent as usize].count;
	let left_right_nid = self.right_cell(left_nid).unwrap();
	self.rotate_left_raw(left_nid, left_right_nid);
	self.rotate_right_raw(parent, left_right_nid);
	assert!(old_count == self.tree[left_right_nid as usize].count);
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

    fn insert_at_cell_prev(&mut self, nid: i32, new_nid: i32) -> (i32, i32) {
	assert!(!self.is_set(new_nid));
	if nid == -1 {
	    self.insert_cell_root(new_nid);
	    return (-1, 0);
	}
	assert!(self.is_set(nid));
	// attach prev:   parent \
	//       (left -> new) <- nid -> right
	self.tree[new_nid as usize].count = 1; // init
	let parent_dir = if let Some(left) = self.left_cell(nid) {
	    (self.sub_last(left), 1)
	} else {
	    (nid, -1)
	};
	self.attach_parent(parent_dir, new_nid);
	parent_dir
    }
    
    pub fn insert_at_cell(&mut self, nid: i32, new_nid: i32) {
	let parent_dir = self.insert_at_cell_prev(nid, new_nid);
	self.update_parent_bf_insert(new_nid, parent_dir);
    }
    
    fn update_parent_bf_insert(&mut self, new_nid: i32, parent_dir: (i32, i32)) {
	let mut current = new_nid;
	let (mut parent, mut dir) = parent_dir;
	//println!("Before adjust: {} -> {:?}", self.root, self.tree);
	while parent != -1 {
	    //println!("Balance parent {}(bf: {}) dir {} node {} (bf: {})", parent, self.node(parent).bf, dir, current, self.node(current).bf);
	    if dir < 0 {
		if self.node(parent).bf < 0 {
		    if self.node(current).bf > 0 {
			self.rotate_left_right(parent, current);
		    } else {
			//println!("Rotate right {} {}", parent, current);
			self.rotate_right(parent, current);
		    }
		    break;
		} else {
		    self.node_mut(parent).bf -= 1;
		    //println!("Adjust parent {}(bf: {})", parent, self.node(parent).bf);
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
		    //println!("Adjust parent {}(bf: {})", parent, self.node(parent).bf);
		    if self.node(parent).bf == 0 {
			break;
		    }
		}
	    } else {
		unreachable!();
	    }
	    current = parent;
	    (parent, dir) = self.get_parent_dir(current);
	}
    }

    fn remove_at_cell_prev(&mut self, nid: i32) -> (i32, i32) {
	assert!(self.is_connected(nid));
	let parent_dir = self.detach_parent(nid);
	let nid_left = self.detach_left(nid);
	let nid_right = self.detach_right(nid);
	let nid_bf = self.node(nid).bf;
	*self.node_mut(nid) = OrdNode::default();
	if nid_left == -1 && nid_right == -1 {
	    self.attach_parent(parent_dir, -1);
	    parent_dir
	} else if nid_left == -1 {
	    self.attach_parent(parent_dir, nid_right);
	    parent_dir
	} else if nid_right == -1 {
	    self.attach_parent(parent_dir, nid_left);
	    parent_dir
	} else {
	    let replace = self.sub_last(nid_left);
	    if replace == nid_left {
		self.attach_right(replace, nid_right);
		self.attach_parent(parent_dir, replace);
		self.node_mut(replace).bf = nid_bf;
		(replace, -1)
	    } else {
		let parent_replace = self.detach_parent(replace);
		let replace_left = self.detach_left(replace);
		self.attach_right(parent_replace.0, replace_left);
		self.attach_left(replace, nid_left);
		self.attach_right(replace, nid_right);
		self.attach_parent(parent_dir, replace);
		self.node_mut(replace).bf = nid_bf;
		parent_replace
	    }
	}
    }

    fn update_parent_bf_remove(&mut self, parent_dir: (i32, i32)) {
	let (mut parent, mut dir) = parent_dir;
	//println!("Before adjust: {:?}", parent_dir);
	while parent != -1 {
	    //println!("Balance parent {}(bf: {}) dir {} node {} (bf: {})", parent, self.node(parent).bf, dir, current, self.node(current).bf);
	    let current;
	    if dir < 0 {
		if self.node(parent).bf > 0 {
		    let right = self.right_cell(parent).unwrap();
		    let bf = self.node(right).bf;
		    if bf < 0 {
			current = self.rotate_right_left(parent, right);
		    } else {
			//println!("Rotate right {} {}", parent, current);
			current = self.rotate_left(parent, right);
		    }
		    if bf == 0 {
			break;
		    }
		} else {
		    //println!("Adjust parent {}(bf: {})", parent, self.node(parent).bf);
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
		    //println!("Adjust parent {}(bf: {})", parent, self.node(parent).bf);
		    if self.node(parent).bf == -1 {
			break;
		    }
		    current = parent;
		}
	    } else {
		unreachable!();
	    }
	    (parent, dir) = self.get_parent_dir(current);
	}
    }

    pub fn remove_at_cell(&mut self, nid: i32) {
	let parent_dir = self.remove_at_cell_prev(nid);
	self.update_parent_bf_remove(parent_dir);
    }

}

impl<T: std::fmt::Debug> std::iter::FromIterator<T> for OrdTree<T> {
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = T>
    {
 	let mut tree = OrdTree::<T>::new();
	tree.root = -1;
	tree.elts = iter.into_iter().collect();
	let len = tree.elts.len();
	tree.tree = (0..len).map(|_| OrdNode::default()).collect();
	for cell in (0..len).rev() {
	    tree.insert_cell(0, cell as i32);
	}
	tree
    }
}

pub struct OrdTreeIterator<'a, T> {
    onto: &'a OrdTree<T>,
    next_cell: Option<i32>
}

impl<T> OrdTree<T> {
    fn iter(&self) -> OrdTreeIterator<'_, T> {
        OrdTreeIterator {
            onto: self,
            next_cell: self.first_cell()
        }
    }
}

impl<'a, T> Iterator for OrdTreeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
	if let Some(cell) = self.next_cell {
	    self.next_cell = self.onto.next_cell(cell);
	    Some(&self.onto.elts[cell as usize])
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

impl<T: std::marker::Copy>OrdTree<T> {
    pub fn to_vec(self) -> Vec<T> {
	self.into_iter().copied().collect()
    }

    pub fn as_vec(&self) -> Vec<T> {
	self.into_iter().copied().collect()
    }
}

impl<T: std::fmt::Debug> OrdTree<T> {

    #[allow(dead_code)]
    fn dump_cell(&self, nid: i32) {
	let node = &self.tree[nid as usize];
	let left_str = if let Some(left) = self.left_cell(nid) {
	    let node = &self.tree[left as usize];
	    format!("left {}({})[{}]", left, node.count, node.bf)
	} else {
	    "_".to_string()
	};
	let right_str = if let Some(right) = self.right_cell(nid) {
	    let node = &self.tree[right as usize];
	    format!("right {}({})[{}]", right, node.count, node.bf)
	} else {
	    "_".to_string()
	};
	println!("node {}({})[{}] = {:?}: {} {}" , nid, node.count, node.bf, self.get_val(nid), left_str, right_str);
    }

    #[allow(dead_code)]
    fn dump_tree(&self) {
	let mut stack = Vec::<i32>::new();
	stack.push(self.root);
	while !stack.is_empty() {
	    let nid = stack.pop().unwrap();
	    if nid == -1 {
		continue;
	    }
	    self.dump_cell(nid);
	    if let Some(left) = self.left_cell(nid) {
		stack.push(left);
	    }
	    if let Some(right) = self.right_cell(nid) {
		stack.push(right);
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
	    if tree.get_cell(i) !=  i as i32 {
		tree.dump_tree();
		assert_eq!(tree.get_cell(i), i as i32);
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
	for (rem, v) in [(2, -3),
			 (5, 4),
			 (3, -2),
			 (0, 1),
			 (1, 3),
			 (1, 0),
			 (0, 2)] {
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
	for (rem, ins) in [(0, 1),
			   (0, 2),
			   (1, 4),
			   (2, 5),
			   (2, 0),
			   (4, 4),
			   (6, 4)] {
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
	let index_positions = |positions: &[usize], idx: usize | positions.iter().position(|&x| x == idx).unwrap();
	let positions_val = |positions: &[usize], vec: &[i64] | positions.iter().map(|&x| vec[x]).collect::<Vec<i64>>();
	let mut positions = (0..input.len()).collect::<Vec<usize>>();
	let mut tree = input.iter().copied().collect::<OrdTree<i64>>();
	let len = input.len() as i64;
	for _r in 0..rounds {
	    for (idx, val) in input.iter().enumerate() {
		let pos = tree.index_cell(idx as i32);
		let v_pos = index_positions(&positions, idx);
		assert!(pos == v_pos);
		let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
		positions.remove(pos);
		let tree_before_remove = tree.clone();
		let parent_dir = tree.remove_at_cell_prev(idx as i32);
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
		tree.insert_cell(target_pos, idx as i32);
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
	let mut tree = data.trim().split('\n').map(|x| x.parse::<i64>().unwrap() * key).collect::<OrdTree<i64>>();
	let len = tree.len() as i64;
	for _r in 0..rounds {
	    for (idx, val) in tree.clone().iter().enumerate() {
		let pos = tree.index_cell(idx as i32);
		let target_pos = (pos as i64 + val).rem_euclid(len - 1) as usize;
		tree.remove_at_cell(idx as i32);
		tree.insert_cell(target_pos, idx as i32);
	    }
	}
	let pos_0 = tree.iter().take_while(|&x| *x != 0).count();
	assert_eq!(pos_0, 3697);
    }
    
}
