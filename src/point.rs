use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn zero() -> Self {
        Point::new(0, 0)
    }

    #[inline]
    pub fn one() -> Self {
        Point::new(1, 1)
    }

    #[inline]
    pub fn square_distance(first: Point, second: Point) -> i32 {
        let d = first - second;
        d.x * d.x + d.y * d.y
    }

    #[inline]
    pub fn distance(first: Point, second: Point) -> f32 {
        (Point::square_distance(first, second) as f32).sqrt()
    }

    #[inline]
    pub fn dot(first: Point, second: Point) -> i32 {
        first.x * second.x + first.y * second.y
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_square_distance() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        assert_eq!(Point::square_distance(p1, p2), 2);
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let d = 2.0 as f32;
        assert_eq!(Point::distance(p1, p2), d.sqrt());
    }

    #[test]
    fn test_add() {
        assert_eq!(Point::new(1, 2) + Point::new(3, 4), Point::new(4, 6));
    }

    fn test_add_assign() {
        let mut p = Point::new(1, 2);
        p += Point::new(3, 4);
        assert_eq!(p, Point::new(4, 6));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Point::new(1, 2) - Point::new(3, 4), Point::new(-2, -2));
    }

    #[test]
    fn test_sub_assign() {
        let mut p = Point::new(1, 2);
        p -= Point::new(3, 4);
        assert_eq!(p, Point::new(-2, -2));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Point::new(1, 2) * 2, Point::new(2, 4));
    }

    #[test]
    fn test_mul_assign() {
        let mut p = Point::new(1, 2);
        p *= 4;
        assert_eq!(p, Point::new(4, 8));
    }

    #[test]
    fn test_mul2() {
        assert_eq!(2 * Point::new(1, 2), Point::new(2, 4));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Point::new(1, 2), Point::new(-1, -2));
    }
}
