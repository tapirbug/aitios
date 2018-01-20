
use ::cgmath::Vector3;

pub trait Position {
    fn position(&self) -> Vector3<f32>;
}

impl Position for Vector3<f32> {
    fn position(&self) -> Vector3<f32> {
        *self
    }
}

pub trait Normal {
    fn normal(&self) -> Vector3<f32>;
}

// TODO normal, tangent, binormal...
