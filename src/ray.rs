use scene::Scene;
use point::Point;
use vector::Vector3;

pub struct Ray {
  pub origin: Point,
  pub direction: Vector3,
}

impl Ray {
  pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
    assert!(scene.width > scene.height);

    let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);
    let sensor_x = (get_sensor_x(x, scene.width) * aspect_ratio) * fov_adjustment;
    let sensor_y = (get_sensor_y(y, scene.height)) * fov_adjustment;

    Ray {
      origin: Point::zero(),
      direction: Vector3 {
        x: sensor_x,
        y: sensor_y,
        z: -1.0,
      }.normalize(),
    }
  }
}

fn get_sensor_x(x: u32, width: u32) -> f64 {
  let pixel_center = x as f64 + 0.5;
  let normalized_to_width = pixel_center / width as f64;
  let adjusted_to_screen_pos = (normalized_to_width * 2.0) - 1.0;
  adjusted_to_screen_pos
}

fn get_sensor_y(y: u32, height: u32) -> f64 {
  let pixel_center = y as f64 + 0.5;
  let normalized_to_height = pixel_center / height as f64;
  let adjusted_to_screen_pos = 1.0 - (normalized_to_height * 2.0);
  adjusted_to_screen_pos
}
