extern crate cgmath;

pub use cgmath::{Deg, Matrix4, PerspectiveFov, Vector4, Vector3};

mod render {
	struct Camera {
		viewMat: Matrix4<f32>,
		projMat: PerspectiveFov<f32>,
		position: Vector3<f32>,
		direction: Vector3<f32>,
		rotation::Quaternion<f32>,
		upVec: Vector4<f32>,
	}

	impl Camera {
		pub fn getView() -> Matrix4<f32> {
			return viewMat
		}

		pub fn getProj() -> PerspectiveFov<f32> {
			return projMat
		}

		pub fn getDir() -> Vector3<f32> {
			return direction
		}

		pub fn getPos() -> Vector3<f32> {
			return position
		}

		pub fn Update() {
			
		}

		pub fn Move(dir: Vector3<f32>, speed: f32) {

		}

		pub fn MoveYAxis(speed: f32) {

		}

		pub fn ResizeCamera(width: f32, height: f32) {

		}

		pub fn MouseRotate(x: f32, y: f32) {

		}
	}
}