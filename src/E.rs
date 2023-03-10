use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq)]
#[allow(dead_code)]
pub struct E {
    weight: i32,
    node: i32,
}
impl Ord for E {
    fn cmp(&self, other: &E) -> Ordering {
        if self.weight == other.weight {
            Ordering::Equal
        } else if self.weight < other.weight {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for E {
    fn partial_cmp(&self, other: &E) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for E {
    fn eq(&self, other: &E) -> bool {
        self.weight == other.weight
    }
}
