use std::cmp::Ordering;

#[derive(Default, Debug, Eq)]
pub enum RadixTree {
    /// This variant represents the empty tree.
    #[default]
    Empty,
    /// This variant stores a string in its entirety for easy lookup.
    Leaf(String),
    /// This variant stores the prefix, and all strings that start with that prefix.
    Node(String, Vec<RadixTree>),
}

impl Ord for RadixTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for RadixTree {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RadixTree::Empty, RadixTree::Empty) => true,
            (RadixTree::Empty, _) => false,
            (_, RadixTree::Empty) => false,
            (RadixTree::Leaf(a), RadixTree::Leaf(b)) => a == b,
            (RadixTree::Leaf(_), RadixTree::Node(_, _)) => false,
            (RadixTree::Node(_, _), RadixTree::Leaf(_)) => false,
            // must preserve order
            (RadixTree::Node(a, lhs), RadixTree::Node(b, rhs)) => a == b && lhs == rhs,
        }
    }
}

impl PartialOrd for RadixTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RadixTree::Empty, RadixTree::Empty) => Some(Ordering::Equal),
            (RadixTree::Empty, _) => Some(Ordering::Less),
            (_, RadixTree::Empty) => Some(Ordering::Greater),
            (RadixTree::Leaf(a), RadixTree::Leaf(b)) => Some(a.cmp(b)),
            (RadixTree::Leaf(_), RadixTree::Node(_, _)) => Some(Ordering::Less),
            (RadixTree::Node(_, _), RadixTree::Leaf(_)) => Some(Ordering::Greater),
            (RadixTree::Node(a, _), RadixTree::Node(b, _)) => Some(a.cmp(b)),
        }
    }
}

impl RadixTree {
    /// Lookup a string in the tree.
    pub fn lookup(&self, string: &str) -> bool {
        self.lookup_inner(string, 0)
    }

    fn lookup_inner(&self, string: &str, pos: usize) -> bool {
        let suffix = &string[pos..];
        match self {
            // empty tree, no match
            RadixTree::Empty => false,
            // compare suffixes of the string and the leaf
            RadixTree::Leaf(item) => item.get(pos..) == Some(&string[pos..]),
            // compare suffix of the string and the prefix of the node
            RadixTree::Node(prefix, children) => {
                if suffix.starts_with(prefix) {
                    // recurse
                    children
                        .iter()
                        .any(|child| child.lookup_inner(string, pos + prefix.len()))
                } else {
                    false
                }
            }
        }
    }

    pub fn insert(&mut self, string: &str) -> bool {
        self.insert_inner(string, 0)
    }

    fn insert_inner(&mut self, string: &str, pos: usize) -> bool {
        todo!()
        // match self {
        //     RadixTree::Empty => {
        //         *self = RadixTree::Leaf(string.to_string());
        //     }
        //     // find the longest common prefix and split the node
        //     RadixTree::Leaf(leaf) => {
        //         let common_prefix = leaf
        //             .chars()
        //             .zip(string[pos..].chars())
        //             .take_while(|(a, b)| a == b)
        //             .count();
        //         let common_prefix = &leaf[..common_prefix];
        //         let mut children = vec![
        //             RadixTree::Leaf(leaf[common_prefix.len()..].to_string()),
        //             RadixTree::Leaf(string[pos + common_prefix.len()..].to_string()),
        //         ];
        //         // sort children to preserve order
        //         children.sort();
        //         *self = RadixTree::Node(common_prefix.to_string(), children);
        //     },
        // 	// if we have a node, then we need to find the longest common prefix
        // 	// from all children
        //     RadixTree::Node(prefix, children) => {
        // 		todo!()
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::RadixTree;

    #[test]
    fn lookup_empty() {
        let tree = RadixTree::Empty;
        assert_eq!(tree.lookup("foo"), false);
    }

    #[test]
    fn lookup_leaf() {
        let tree = RadixTree::Leaf("foo".to_string());
        assert!(tree.lookup("foo"));
        assert!(!tree.lookup("bar"));
    }

    #[test]
    fn lookup_node() {
        let tree = RadixTree::Node(
            "foo".to_string(),
            vec![
                RadixTree::Leaf("foobar".to_string()),
                RadixTree::Leaf("foobaz".to_string()),
            ],
        );

        assert!(tree.lookup("foobar"));
        assert!(tree.lookup("foobaz"));
        assert!(!tree.lookup("foo"));
    }

    #[test]
    fn insert_empty() {
        let mut tree = RadixTree::Empty;
        tree.insert("foo");
        assert_eq!(tree, RadixTree::Leaf("foo".to_string()));
    }
}
