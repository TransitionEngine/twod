use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::numeric::Numeric;

pub struct Matrix<T: Numeric> {
    pub m11: T,
    pub m12: T,
    pub m21: T,
    pub m22: T,
}
impl<T: Numeric> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            m11: T::default(),
            m12: T::default(),
            m21: T::default(),
            m22: T::default(),
        }
    }
}
impl<T: Numeric> Matrix<T> {
    pub fn new(m11: T, m12: T, m21: T, m22: T) -> Self {
        Self {
            m11,
            m12,
            m21,
            m22,
        }
    }

    pub fn scalar(scalar: T) -> Self {
        Self {
            m11: scalar,
            m12: scalar,
            m21: scalar,
            m22: scalar,
        }
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn transpose(&self) -> Self {
        Self {
            m11: self.m11,
            m12: self.m21,
            m21: self.m12,
            m22: self.m22,
        }
    }
}
impl<T: Numeric + Into<f32> + From<f32>> Matrix<T> {
    pub fn unity() -> Self {
        Self {
            m11: T::from(1.0),
            m12: T::from(0.0),
            m21: T::from(0.0),
            m22: T::from(1.0),
        }
    }
    pub fn inverse(&self) -> Option<Matrix<T>> {
        let det = self.determinant().into();
        if det == 0.0 {
            return None;
        }
        let inv_det = (1.0 / det).into();
        Some(Self {
            m11: self.m22 * inv_det,
            m12: <f32 as Into<T>>::into(-1.0) * self.m12 * inv_det,
            m21: <f32 as Into<T>>::into(-1.0) * self.m21 * inv_det,
            m22: self.m11 * inv_det,
        })
    }
}

impl<T: Numeric> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}
impl<T: Numeric> Mul<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}
impl<T: Numeric> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}
impl<T: Numeric> Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}
impl<T: Numeric> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * rhs;
    }
}
impl<T: Numeric> MulAssign<&Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Matrix<T>) {
        *self = &*self * rhs;
    }
}

macro_rules! impl_matric_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op for Matrix<T> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                }
            }
        }
        impl<T: Numeric> $op<&Matrix<T>> for Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: &Self) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                }
            }
        }
        impl<T: Numeric> $op for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                }
            }
        }
        impl<T: Numeric> $op<Matrix<T>> for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: Matrix<T>) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                }
            }
        }
        impl<T: Numeric> $op_assign for Matrix<T> {
            fn $method_assign(&mut self, rhs: Self) {
                self.m11 = self.m11.$method(rhs.m11);
                self.m12 = self.m12.$method(rhs.m12);
                self.m21 = self.m21.$method(rhs.m21);
                self.m22 = self.m22.$method(rhs.m22);
            }
        }
        impl<T: Numeric> $op_assign<&Matrix<T>> for Matrix<T> {
            fn $method_assign(&mut self, rhs: &Self) {
                self.m11 = self.m11.$method(rhs.m11);
                self.m12 = self.m12.$method(rhs.m12);
                self.m21 = self.m21.$method(rhs.m21);
                self.m22 = self.m22.$method(rhs.m22);
            }
        }
    };
    () => {};
}
impl_matric_op!(Add, AddAssign, add, add_assign);
impl_matric_op!(Sub, SubAssign, sub, sub_assign);

macro_rules! impl_scalar_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op<T> for Matrix<T> {
            type Output = Self;

            fn $method(self, rhs: T) -> Self::Output {
                Self {
                    m11: self.m11.$method(rhs),
                    m12: self.m12.$method(rhs),
                    m21: self.m21.$method(rhs),
                    m22: self.m22.$method(rhs),
                }
            }
        }
        impl<T: Numeric> $op<T> for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: T) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs),
                    m12: self.m12.$method(rhs),
                    m21: self.m21.$method(rhs),
                    m22: self.m22.$method(rhs),
                }
            }
        }
        impl<T: Numeric> $op_assign<T> for Matrix<T> {
            fn $method_assign(&mut self, rhs: T) {
                self.m11 = self.m11.$method(rhs);
                self.m12 = self.m12.$method(rhs);
                self.m21 = self.m21.$method(rhs);
                self.m22 = self.m22.$method(rhs);
            }
        }
    };
    () => {};
}

impl_scalar_op!(Mul, MulAssign, mul, mul_assign);
impl_scalar_op!(Div, DivAssign, div, div_assign);
