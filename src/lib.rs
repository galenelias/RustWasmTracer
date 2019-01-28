extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::ops;

cfg_if! {
	// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
	// allocator.
	if #[cfg(feature = "wee_alloc")] {
		extern crate wee_alloc;
		#[global_allocator]
		static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
	}
}

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

macro_rules! console_log {
	// Note that this is using the `log` function imported above during
	// `bare_bones`
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
	a: u8,
	r: u8,
	g: u8,
	b: u8
}

#[derive(Debug, Clone, Copy)]
struct Vector3 {
	x: f64,
	y: f64,
	z: f64,
}

impl Vector3 {
	fn zero() -> Vector3 {
		Vector3 {x: 0.0, y: 0.0, z: 0.0 }
	}

	fn magnitude_squared(&self) -> f64 {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}

	fn magnitude(&self) -> f64 {
		self.magnitude_squared().sqrt()
	}

	fn normalize(&self) -> Vector3 {
		let mag = self.magnitude();
		if mag > 0.0 {
			Vector3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
		} else {
			self.clone()
		}
	}

	fn dot_product(&self, other: &Vector3) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

impl ops::Add<Vector3> for Vector3 {
	type Output = Vector3;

	fn add(self, _rhs: Vector3) -> Vector3 {
		Vector3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
	}
}

impl ops::Sub<Vector3> for Vector3 {
	type Output = Vector3;

	fn sub(self, _rhs: Vector3) -> Vector3 {
		Vector3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
	}
}


impl ops::Mul<f64> for Vector3 {
	type Output = Vector3;

	fn mul(self, _rhs: f64) -> Vector3 {
		Vector3 { x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs }
	}
}

impl ops::Div<f64> for Vector3 {
	type Output = Vector3;

	fn div(self, _rhs: f64) -> Vector3 {
		Vector3 { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs }
	}
}

struct Ray {
	origin: Vector3,
	direction: Vector3,
}

#[wasm_bindgen]
pub struct Scene {
	width: usize,
	height: usize,
	pixels: Vec<u8>,
	time: f64,
}

const SURF_DIST: f64 = 0.01;

impl Scene {

	fn get_index(&self, row: usize, column: usize) -> usize {
		(row * self.width + column) as usize
	}

	// fn get_cell(&self, row: usize, column: usize) -> Cell {
	// 	self.cells[self.get_index(row, column)]
	// }

	fn get_dist(&self, pt: Vector3) -> f64 {
		let sphere = Sphere { center: Vector3 { x: 0.0, y: 1.0, z: 6.0 }, radius: 1.0 };

		let sphere_dist = sphere.dist_to(pt);
		let plane_dist = pt.y;

		let d = sphere_dist.min(plane_dist);
		return d;
	}

	fn get_normal(&self, pt: Vector3) -> Vector3 {
		let dist = self.get_dist(pt);
		let normal = Vector3 {
			x: dist - self.get_dist(pt - Vector3{ x: 0.01, y: 0.0, z: 0.0}),
			y: dist - self.get_dist(pt - Vector3{ x: 0.00, y: 0.1, z: 0.0}),
			z: dist - self.get_dist(pt - Vector3{ x: 0.00, y: 0.0, z: 0.1}),
		};
		normal.normalize()
	}

	fn get_light(&self, pt: Vector3) -> f64 {
		let t = (self.time / 1000.0) * 2.0;
		let light_pos = Vector3 { x: 2.0 * t.sin(), y: 5.0, z: 6.0 + 2.0 * t.cos()};
		// console_log!("Camera: {:?}", light_pos);

		// let light_pos = Vector3 { x: 1.0, y: 5.0, z: 6.0 };
		let lv = (light_pos - pt).normalize();
		let normal = self.get_normal(pt);
		let mut diffuse = normal.dot_product(&lv);

		let dl = self.ray_march(&Ray{ origin: pt + normal * (2.0 * SURF_DIST), direction: lv});
		if dl < (light_pos - pt).magnitude() {
			diffuse *= 0.1;
		}
		diffuse
	}

	fn ray_march(&self, ray: &Ray) -> f64 {
		const MAX_STEPS: usize = 100;
		const MAX_DIST: f64 = 100.0;

		let mut d0 = 0.0;

		for _ in 0..MAX_STEPS {
			let p = ray.origin + ray.direction * d0;
			let ds = self.get_dist(p);
			d0 += ds;

			if d0 > MAX_DIST || ds < SURF_DIST {
				break;
			}
		}
		return d0;
	}

	fn render(&self, cells : &mut Vec<u8>) {
		for y in 0..self.height {
			for x in 0..self.width {
				let prime_ray = create_prime(x, y, self);

				let rm = self.ray_march(&prime_ray);
				let pt = prime_ray.origin + prime_ray.direction * rm;
				let diffuse_light = self.get_light(pt);
				let color = diffuse_light;
				let color = if color >= 1.0 { 255 } else { (color * 256.0) as u8 };

				let idx = self.get_index(y, x);
				cells[idx * 4 + 0] = color;
				cells[idx * 4 + 1] = color;
				cells[idx * 4 + 2] = color;
				cells[idx * 4 + 3] = 255;
			}
		}
	}
}

fn create_prime(x: usize, y: usize, scene: &Scene) -> Ray {
	let fov : f64 = 45.0;
	let fov_adjustment = (fov.to_radians() / 2.0).tan();
	let aspect_ratio = scene.width() as f64 / scene.height() as f64;
	let sensor_x = ((((x as f64 + 0.5) / scene.width() as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
	let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height() as f64) * 2.0) * fov_adjustment;

	Ray {
		origin: Vector3 {
			x: 0.0,
			y: 1.0,
			z: 0.0,
		},
		direction: Vector3 {
			x: sensor_x,
			y: sensor_y,
			z: 1.0,
		}
		.normalize(),
	}
}

struct Sphere {
	center: Vector3,
	radius: f64,
}

impl Sphere {
	fn dist_to(&self, vec: Vector3) -> f64 {
		(self.center - vec).magnitude() - self.radius
	}
}


 #[wasm_bindgen]
impl Scene {
	pub fn tick(&mut self) {
		let mut next = self.pixels.clone();
		self.render(&mut next);
		self.pixels = next;
		self.time = web_sys::window()
			.expect("should have a Window")
			.performance()
			.expect("should have a Performance")
			.now();

	}

	pub fn new() -> Scene {
		let width = 640;
		let height = 480;
		// let width = 64;
		// let height = 48;
		let pixels = vec![0; width * height * 4];

		Scene {
			width,
			height,
			pixels,
			time: 0.0,
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

   pub fn cells(&self) -> *const u8 {
		self.pixels.as_ptr()
	}
}

