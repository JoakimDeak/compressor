use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    pub byte: Option<u8>,
    pub count: u32,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new_external(byte: u8, count: u32) -> Self {
        Node {
            byte: Some(byte),
            count,
            left: None,
            right: None,
        }
    }
    pub fn new_internal(left: Self, right: Self) -> Self {
        Node {
            byte: None,
            count: left.count + right.count,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}
