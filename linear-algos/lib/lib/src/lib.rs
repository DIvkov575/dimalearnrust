use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq)]
#[allow(dead_code)]

pub struct E {
    pub weight: u32,
    pub node: u32,
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
// const V: [[u32]; 6] = [[1, 2], [2, 3], [5, 6], [6, 7], [], [], []];
// const W: [[u32]; 6] = [[4, 3], [5, 6], [7], [3], [], [], []];

// const G: Box<HashMap<u32, Vec<E>>> = Box::new(HashMap::from([
//     (
//         0,
//         Vec::from([E { node: 1, weight: 4 }, E { node: 2, weight: 3 }]),
//     ),
//     (
//         1,
//         Vec::from([E { node: 2, weight: 5 }, E { node: 3, weight: 6 }]),
//     ),
//     (2, Vec::from([E { node: 4, weight: 7 }])),
//     (3, Vec::from([E { node: 5, weight: 3 }])),
//     (4, Vec::from([])),
//     (5, Vec::from([])),
// ]));
