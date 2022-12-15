#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    pub a: i64,
    pub b: i64,
}

impl Interval {
    pub fn new(a: i64, b: i64) -> Interval {
        debug_assert!(a <= b);
        Interval { a, b }
    }

    /// Test if `other` is a subset of `self`.
    pub fn contains(&self, other: &Interval) -> bool {
        self.a <= other.a && other.b <= self.b
    }

    /// Test if the intersection is non-empty.
    pub fn overlaps(&self, other: &Interval) -> bool {
        let (left, right) = if self <= other {
            (self, other)
        } else {
            (other, self)
        };
        left.b >= right.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let x = Interval::new(2, 5);
        let y = Interval::new(3, 5);
        assert_eq!(true, x.contains(&y));
        assert_eq!(false, y.contains(&x));

        let z = Interval::new(3, 6);
        assert_eq!(false, x.contains(&z));
    }

    #[test]
    fn test_overlaps() {
        let x = Interval::new(2, 5);
        let y = Interval::new(3, 6);
        assert_eq!(true, x.overlaps(&y));
        assert_eq!(true, y.overlaps(&x));
    }
}
