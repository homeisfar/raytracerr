use std::ops::{Add, Sub};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Vector3;

    fn sub(self, other: &'b Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        self.normal().sqrt()
    }
    pub fn normal(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    pub fn normalize_to_frame(&self) -> Vector3 {
        let inv_len = self.length().recip();
        Vector3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }
    pub fn dot(&self, other: &Vector3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

/***************************/
/*    Ray tracer structs   */
/***************************/

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime_ray(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        // Assume width > height
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let camera_sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let camera_sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: camera_sensor_x,
                y: camera_sensor_y,
                z: -1.0,
            }.normalize_to_frame(),
        }
    }
}


/***************************/
/* Scene object properties */
/***************************/

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        // let length = Vector3 {
        //     x: self.center.x - ray.origin.x,
        //     y: self.center.y - ray.origin.y,
        //     z: self.center.z - ray.origin.z,
        // };
        let length: Vector3 = &self.center - &ray.origin;

        let adjacent2 = length.dot(&ray.direction);

        let d2 = length.dot(&length) - (adjacent2.powf(2.0));

        d2 < (self.radius.powf(2.0))
        // false
    }
}