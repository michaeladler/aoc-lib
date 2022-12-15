#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Point2D { x, y }
    }

    /// Compute the Manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: &Point2D) -> i64 {
        (self.y - other.y).abs() + (self.x - other.x).abs()
    }

    /// Compute the Euclidean squared distance between `self` and `other`.
    pub fn euclidean_squared(&self, other: &Point2D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx) + (dy * dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan() {
        let p = Point2D::new(0, 0);
        assert_eq!(30, p.manhattan(&Point2D { x: 10, y: 20 }));
    }

    #[test]
    fn test_euclidean_squared() {
        let p = Point2D::new(0, 0);
        assert_eq!(500, p.euclidean_squared(&Point2D { x: 10, y: 20 }));
    }
}
