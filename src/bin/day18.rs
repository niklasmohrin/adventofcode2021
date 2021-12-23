use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    pub fn add(self, other: Self) -> Self {
        let mut tmp = self.concat(other);
        tmp.reduce();
        tmp
    }

    fn concat(self, other: Self) -> Self {
        SnailfishNumber::Pair(Box::new(self), Box::new(other))
    }

    fn reduce(&mut self) {
        loop {
            while self.explode() {}
            if !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self) -> bool {
        // a bit hacky, but I think this is safe
        let mut left: Option<*mut Self> = None;
        let mut right: Option<*mut Self> = None;
        let mut to_add = None;

        self.nodes_do_mut(&mut |node, pairs_around_node| {
            if node.is_pair() {
                if to_add.is_none() && pairs_around_node >= 4 {
                    to_add = Some(match node {
                        Self::Pair(l, r) => (l.regular_value(), r.regular_value()),
                        _ => unreachable!(),
                    });
                    *node = Self::Regular(0);
                }
            } else {
                if to_add.is_none() {
                    left = Some(node as *mut _);
                } else if to_add.is_some() && right.is_none() {
                    right = Some(node as *mut _);
                }
            }
        });

        // probably has no effect, but let's not have this around when dereferencing those pointers
        drop(self);

        if let Some((l_add, r_add)) = to_add {
            if let Some(left) = left {
                // SAFETY:
                // the pointer to this object is still valid because `left` is not a child of the
                // exploded node, because the children of the exploded node appear after it in the
                // iteration
                let left = unsafe { &mut *left };
                *left = Self::Regular(left.regular_value() + l_add);
            }
            if let Some(right) = right {
                // SAFETY:
                // the pointer to this object is still valid because it was acquired _after_ the
                // destruction of the exploded node and it was not a child of `left`, because
                // we know that `left.is_regular()`
                let right = unsafe { &mut *right };
                *right = Self::Regular(right.regular_value() + r_add);
            }
        }

        to_add.is_some()
    }

    fn split(&mut self) -> bool {
        let mut done = false;

        self.leafs_do_mut(&mut |leaf: &mut Self, _| {
            if !done {
                let v = leaf.regular_value();
                if v >= 10 {
                    *leaf = SnailfishNumber::Pair(
                        Box::new(SnailfishNumber::Regular(v / 2)),
                        Box::new(SnailfishNumber::Regular(v - (v / 2))),
                    );
                    done = true;
                }
            }
        });

        done
    }

    /// Pre-order traversal
    fn nodes_do_mut(&mut self, f: &mut impl FnMut(&mut Self, usize)) {
        fn helper(
            node: &mut SnailfishNumber,
            f: &mut impl FnMut(&mut SnailfishNumber, usize),
            pairs_around_node: usize,
        ) {
            f(node, pairs_around_node);
            if let SnailfishNumber::Pair(left, right) = node {
                helper(left, f, pairs_around_node + 1);
                helper(right, f, pairs_around_node + 1);
            }
        }

        helper(self, f, 0)
    }

    fn leafs_do_mut(&mut self, f: &mut impl FnMut(&mut Self, usize)) {
        self.nodes_do_mut(&mut |node, pairs_around_node| {
            if node.is_regular() {
                f(node, pairs_around_node);
            }
        });
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Self::Regular(v) => *v,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn regular_value(&self) -> u32 {
        match self {
            Self::Regular(v) => *v,
            _ => unreachable!(),
        }
    }

    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular(..))
    }

    fn is_pair(&self) -> bool {
        matches!(self, Self::Pair(..))
    }

    pub fn from_nested_str(s: &str) -> Self {
        fn parse_from_start(s: &str) -> (SnailfishNumber, usize) {
            match s.chars().next().unwrap() {
                '[' => {
                    let s = &s[1..];

                    let (left, comma_offset) = parse_from_start(s);
                    let s = &s[comma_offset..];

                    assert_eq!(&s[0..1], ",");
                    let s = &s[1..];

                    let (right, end_offset) = parse_from_start(s);
                    let s = &s[end_offset..];
                    assert_eq!(&s[0..1], "]");

                    (
                        SnailfishNumber::Pair(Box::new(left), Box::new(right)),
                        1 + comma_offset + 1 + end_offset + 1,
                    )
                }
                other => (SnailfishNumber::Regular(other.to_digit(10).unwrap()), 1),
            }
        }

        let (res, offset) = parse_from_start(s);
        assert_eq!(offset, s.len());
        res
    }

    pub fn to_nested_str(&self) -> String {
        match self {
            Self::Regular(v) => format!("{}", v),
            Self::Pair(left, right) => {
                format!("[{},{}]", left.to_nested_str(), right.to_nested_str())
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let nums = lines
        .map(|s| SnailfishNumber::from_nested_str(&s.unwrap()))
        .collect_vec();

    // Part 1
    let sum = nums.iter().cloned().reduce(SnailfishNumber::add).unwrap();
    dbg!(sum.to_nested_str());
    dbg!(sum.magnitude());

    // Part 2
    let max_mag = (0..nums.len())
        .cartesian_product(0..nums.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| nums[i].clone().add(nums[j].clone()).magnitude())
        .max()
        .unwrap();
    dbg!(max_mag);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reduce_helper(initial: &str, expected: &str) {
        let mut parsed = SnailfishNumber::from_nested_str(initial);
        parsed.reduce();
        assert_eq!(parsed.to_nested_str(), expected);
    }

    #[test]
    fn reduce1() {
        reduce_helper("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    }
    #[test]
    fn reduce2() {
        reduce_helper("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    }
    #[test]
    fn reduce3() {
        reduce_helper("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn add1() {
        assert_eq!(
            SnailfishNumber::from_nested_str("[[[[4,3],4],4],[7,[[8,4],9]]]")
                .add(SnailfishNumber::from_nested_str("[1,1]"))
                .to_nested_str(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }
}
