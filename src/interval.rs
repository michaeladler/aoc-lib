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

    /// Count (whole) points in the interval.
    /// Example: [1, 3] contains 3 points.
    pub fn len(&self) -> usize {
        (self.b - self.a) as usize + 1
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

    /// Intersect with interval.
    pub fn intersect(&self, interval: &ClosedInterval) -> Option<ClosedInterval> {
        let a = std::cmp::max(self.a, interval.a);
        let b = std::cmp::min(self.b, interval.b);
        if a <= b {
            return Some(ClosedInterval::new(a, b));
        }
        None
    }
}

/// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping
/// intervals, and return an array of the non-overlapping intervals that cover all the intervals in
/// the input.
///
/// Example:
///
/// Input: `[[1,3],[2,6],[8,10],[15,18]]`
/// Output: `[[1,6],[8,10],[15,18]]`
pub fn merge_intervals(intervals: &[ClosedInterval]) -> Vec<ClosedInterval> {
    // see https://leetcode.com/problems/merge-intervals/description/
    // based on https://leetcode.com/problems/merge-intervals/solutions/21223/beat-98-java-sort-start-end-respectively/
    let n = intervals.len();
    let mut starts = Vec::with_capacity(n);
    let mut ends = Vec::with_capacity(n);
    for int in intervals {
        starts.push(int.a);
        ends.push(int.b);
    }
    starts.sort_unstable();
    ends.sort_unstable();
    let (starts, ends) = (starts, ends);
    let mut res = Vec::with_capacity(n);
    let mut i = 0;
    let mut j = 0;
    while i < n {
        if i == n - 1 || unsafe { starts.get_unchecked(i + 1) > ends.get_unchecked(i) } {
            res.push(ClosedInterval::new(starts[j], ends[i]));
            j = i + 1;
        }
        i += 1;
    }
    res
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

    #[test]
    fn test_intersect() {
        let x = ClosedInterval::new(2, 5);
        let y = ClosedInterval::new(3, 6);
        assert_eq!(Some(ClosedInterval::new(3, 5)), x.intersect(&y));
    }

    #[test]
    fn test_merge_intervals() {
        {
            let intervals = vec![
                ClosedInterval::new(1, 3),
                ClosedInterval::new(2, 6),
                ClosedInterval::new(8, 10),
                ClosedInterval::new(15, 18),
            ];
            let result = merge_intervals(&intervals);
            assert_eq!(
                vec![
                    ClosedInterval::new(1, 6),
                    ClosedInterval::new(8, 10),
                    ClosedInterval::new(15, 18)
                ],
                result,
                "example 1"
            );
        }

        {
            let intervals = vec![ClosedInterval::new(1, 4), ClosedInterval::new(4, 5)];
            let result = merge_intervals(&intervals);
            assert_eq!(vec![ClosedInterval::new(1, 5),], result, "example 2");
        }
    }

    #[test]
    fn test_len() {
        assert_eq!(3, ClosedInterval::new(1, 3).len());
        assert_eq!(6, ClosedInterval::new(-2, 3).len());
    }
}
