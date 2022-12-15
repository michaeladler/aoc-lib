#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents the interval [a, b].
pub struct ClosedInterval {
    pub a: i64,
    pub b: i64,
}

impl ClosedInterval {
    pub fn new(a: i64, b: i64) -> ClosedInterval {
        debug_assert!(a <= b);
        ClosedInterval { a, b }
    }

    /// Test if `interval` is a subset of `self`.
    pub fn contains(&self, interval: &ClosedInterval) -> bool {
        self.a <= interval.a && interval.b <= self.b
    }

    /// Test if the intervals are disjoint.
    pub fn disjoint(&self, interval: &ClosedInterval) -> bool {
        let (lhs, rhs) = if self <= interval {
            (self, interval)
        } else {
            (interval, self)
        };
        lhs.b < rhs.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let x = ClosedInterval::new(2, 5);
        let y = ClosedInterval::new(3, 5);
        assert_eq!(true, x.contains(&x));
        assert_eq!(true, x.contains(&y));
        assert_eq!(false, y.contains(&x));

        let z = ClosedInterval::new(3, 6);
        assert_eq!(false, x.contains(&z));
    }

    #[test]
    fn test_overlaps() {
        let x = ClosedInterval::new(2, 5);
        let y = ClosedInterval::new(3, 6);
        let z = ClosedInterval::new(7, 8);
        assert_eq!(false, x.disjoint(&y));
        assert_eq!(false, y.disjoint(&x));
        assert_eq!(true, y.disjoint(&z));
        assert_eq!(true, z.disjoint(&y));
    }
}
