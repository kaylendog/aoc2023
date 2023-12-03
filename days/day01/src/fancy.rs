#[derive(Default, Debug)]
enum PrefixTree {
    // The empty node
    #[default]
    Empty,
    // Contains the string that ends at this node
    Terminal(&'static str),
    // A sub-tree that contains all the strings that start with this prefix
    Branch(&'static str, Vec<PrefixTree>),
}

impl PrefixTree {
    /// Inserts a string into the prefix tree
    pub fn insert(&mut self, item: &'static str) {
        match self {
            PrefixTree::Empty => {
                *self = PrefixTree::Terminal(item);
            }
            PrefixTree::Terminal(_) => {}
        }
    }

    pub fn find(&self, item: &'static str) -> bool {
        let node = self;
        // indices
        let start = 0;
        let len = item.len();
        // walk the tree
        for end in 0..len {
            match node {
                PrefixTree::Empty => return false,
                PrefixTree::Terminal(terminal) => {
                    // only need to check the suffix
                    return terminal[start..] == item[start..];
                }
                PrefixTree::Branch(prefix, children) => {
                    let node = children.iter().find(|child| match child {
                        PrefixTree::Empty => false,
                        PrefixTree::Terminal(terminal) => terminal[start..] == item[start..],
                        PrefixTree::Branch(prefix, _) => item.starts_with(prefix),
                    });
                }
            }
        }

        todo!()
    }
}

fn main() {}
