pub mod point;
pub mod scene;
pub mod ray;
pub mod vector;
pub mod sphere;

use scene::*;
use point::Point;
use ray::Ray;
use sphere::*;

fn main() {
  let sphere = Sphere {
    center: Point::new(0.0, 0.0, -5.0),
    radius: 1.0,
  };
  let scene = Scene {
    width: 48,
    height: 32,
    fov: 90.0,
    sphere,
  };

  let output = render(&scene);

  for line in &output {
    println!("{}", line);
  }
}

fn render(scene: &Scene) -> Vec<String> {
  let mut out: Vec<String> = vec![];

  for y in 0..scene.height {
    let mut line: Vec<char> = vec![];

    for x in 0..scene.width {
      let ray = Ray::create_prime(x, y, scene);

      if scene.sphere.intersect(&ray) {
        line.push('+');
      } else {
        line.push(' ');
      }
    }

    out.push(line.into_iter().collect());
  }
  out
}

#[test]
fn test_can_render_scene() {
  let scene = Scene {
    width: 80,
    height: 60,
    fov: 90.0,
    sphere: Sphere {
      center: Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
      },
      radius: 1.0,
    },
  };

  let text: Vec<String> = render(&scene);
  assert_eq!(scene.height, (text.len() as u32));
  assert_eq!(scene.width, (text[0].len() as u32));
}
