use std::ops::{Add, Sub};
use vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }
  pub fn zero() -> Point {
    Point {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

impl Add<Vector3> for Point {
  type Output = Point;

  fn add(self, other: Vector3) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}
impl Add<Point> for Vector3 {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    other + self
  }
}

impl Sub<Vector3> for Point {
  type Output = Point;

  fn sub(self, other: Vector3) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
impl Sub<Point> for Vector3 {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    other - self
  }
}

impl Sub<Point> for Point {
  type Output = Vector3;

  fn sub(self, other: Point) -> Vector3 {
    Vector3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
