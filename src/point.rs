use core::fmt;
use core::ops;

use num_integer::Integer;
use num_traits::Signed;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Default)]
pub struct Point2D<T = i64>
where
    T: Integer + Signed + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Integer + Signed + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }

    /// Compute the Manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: Point2D<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Compute the Euclidean squared distance between `self` and `other`.
    pub fn euclidean_squared(&self, other: Point2D<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

impl<T> ops::Add<Point2D<T>> for Point2D<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Point2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Sub<Point2D<T>> for Point2D<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: Point2D<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> fmt::Display for Point2D<T>
where
    T: Integer + Signed + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Default)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D { x, y, z }
    }

    /// Compute the Manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: &Point3D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    /// Compute the Euclidean squared distance between `self` and `other`.
    pub fn euclidean_squared(&self, other: &Point3D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl ops::Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan() {
        let p = Point2D::new(0, 0);
        assert_eq!(30, p.manhattan(Point2D { x: 10, y: 20 }));
    }

    #[test]
    fn test_euclidean_squared() {
        let p = Point2D::new(0, 0);
        assert_eq!(500, p.euclidean_squared(Point2D { x: 10, y: 20 }));
    }
}
