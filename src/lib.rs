mod matrix;
mod numeric;
mod vector;

use std::ops::Mul;

pub use matrix::Matrix;
pub use numeric::Numeric;
pub use vector::Vector;

impl<T: Numeric + Into<f32> + From<f32>> Vector<T> {
    pub fn rotate_around(&self, radians: f32) -> Vector<T> {
        let matrix = Matrix::rotation(radians);
        matrix * self
    }
    pub fn rotate_degree_around(&self, degree: f32) -> Vector<T> {
        let matrix = Matrix::rotation_degree(degree);
        matrix * self
    }
}
impl<T: Numeric + Into<f32> + From<f32>> Matrix<T> {
    pub fn rotation(radians: f32) -> Self {
        let c = radians.cos();
        let s = radians.sin();
        Self::new(
            c.into(),
            (-s).into(),
            s.into(),
            c.into(),
        )
    }

    pub fn rotation_degree(degree: f32) -> Self {
        let radians = degree * std::f32::consts::PI / 180.0;
        Self::rotation(radians)
    }
}

impl<T: Numeric> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y,
            self.m12 * rhs.x + self.m22 * rhs.y,
        )
    }
}
impl<T: Numeric> Mul<&Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y,
            self.m12 * rhs.x + self.m22 * rhs.y,
        )
    }
}
impl<T: Numeric> Mul<Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y,
            self.m12 * rhs.x + self.m22 * rhs.y,
        )
    }
}
impl<T: Numeric> Mul<&Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y,
            self.m12 * rhs.x + self.m22 * rhs.y,
        )
    }
}
