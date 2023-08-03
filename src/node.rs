use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    pub byte: Option<u8>,
    pub count: Option<u32>,
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
            count: Some(count),
            left: None,
            right: None,
        }
    }
    pub fn new_external_deserialized(byte: u8) -> Self {
        Node {
            byte: Some(byte),
            count: None,
            left: None,
            right: None,
        }
    }

    pub fn new_internal(left: Self, right: Self) -> Self {
        Node {
            byte: None,
            count: Some(left.count.unwrap_or_default() + right.count.unwrap_or_default()),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    pub fn new_internal_deserialized(left: Self, right: Self) -> Self {
        Node {
            byte: None,
            count: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Node {
    pub fn increment_count(&mut self) {
        if let Some(count) = self.count {
            self.count = Some(count + 1);
        }
    }
}
