use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{Integer, Signed};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point2<S>
where
    S: Integer,
{
    pub x: S,
    pub y: S,
}

macro_rules! point2 {
    ($x:expr, $y:expr) => {
        Point2 { x: $x, y: $y }
    };
}
pub(crate) use point2;

pub type Vector2<S> = Point2<S>;

impl<S> Point2<S>
where
    S: Integer + Copy,
{
    pub fn unit_x() -> Self {
        Point2 {
            x: S::one(),
            y: S::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Point2 {
            x: S::zero(),
            y: S::one(),
        }
    }
}

impl<S> Point2<S>
where
    S: Integer + Copy + Signed,
{
    pub fn norm_1(&self) -> S {
        self.x.abs() + self.y.abs()
    }

    pub fn distance_1(&self, rhs: &Self) -> S {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl<S> Add for Point2<S>
where
    S: Integer,
{
    type Output = Point2<S>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<S> AddAssign for Point2<S>
where
    S: Integer + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<S> Sub for Point2<S>
where
    S: Integer,
{
    type Output = Point2<S>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<S> SubAssign for Point2<S>
where
    S: Integer + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<S> Mul<S> for Vector2<S>
where
    S: Integer + Copy,
{
    type Output = Vector2<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<S> MulAssign<S> for Vector2<S>
where
    S: Integer + Copy,
{
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<S> Div<S> for Vector2<S>
where
    S: Integer + Copy,
{
    type Output = Vector2<S>;

    fn div(self, rhs: S) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<S> DivAssign<S> for Vector2<S>
where
    S: Integer + Copy,
{
    fn div_assign(&mut self, rhs: S) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<S> Neg for Vector2<S>
where
    S: Integer + Signed,
{
    type Output = Vector2<S>;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line2<S>
where
    S: Integer,
{
    pub start: Point2<S>,
    pub end: Point2<S>,
}

impl<S> Line2<S>
where
    S: Integer + Signed + Copy,
{
    pub fn length_1(&self) -> S {
        self.end.distance_1(&self.end)
    }

    fn vec(&self) -> Vector2<S> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let gcd = dx.gcd(&dy);
        match dx.is_zero() || dy.is_zero() {
            true => Vector2 {
                x: dx.signum(),
                y: dy.signum(),
            },
            false => Vector2 {
                x: dx / gcd,
                y: dy / gcd,
            },
        }
    }
}

pub struct Line2Iterator<S>
where
    S: Integer,
{
    point: Point2<S>,
    end: Point2<S>,
    vector: Vector2<S>,
}

impl<S> Iterator for Line2Iterator<S>
where
    S: Integer + Copy,
{
    type Item = Point2<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let passed_end = self.point == self.end;
        match passed_end {
            false => {
                let res = Some(self.point);
                self.point += self.vector;
                res
            }
            true => None,
        }
    }
}

impl<S> IntoIterator for &Line2<S>
where
    S: Integer + Signed + Copy,
{
    type Item = Point2<S>;
    type IntoIter = Line2Iterator<S>;

    fn into_iter(self) -> Self::IntoIter {
        let vector = self.vec();
        Line2Iterator {
            point: self.start,
            end: self.end + vector,
            vector,
        }
    }
}
