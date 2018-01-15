use ray::Ray;
use point::Point;
use vector::Vector3;

pub trait Intersectable {
  fn intersect(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
  pub center: Point,
  pub radius: f64,
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> bool {
    let l: Vector3 = self.center - ray.origin;
    let adj2 = l.dot(&ray.direction);
    let d2 = l.dot(&l) - (adj2 * adj2);
    d2 < (self.radius * self.radius)
  }
}
