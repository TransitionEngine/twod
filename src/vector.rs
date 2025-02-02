use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::numeric::Numeric;

#[derive(PartialEq, Eq)]
pub struct Vector<T: Numeric> {
    pub x: T,
    pub y: T,
}
impl<T: Numeric> Default for Vector<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}
impl<T: Numeric> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn scalar(scalar: T) -> Self {
        Self {
            x: scalar,
            y: scalar,
        }
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn magnitude_squared(&self) -> T {
        self.dot(self)
    }
}

impl<T: Numeric + Into<f32> + From<f32>> Vector<T> {
    pub fn x_axis() -> Self {
        Self {
            x: T::from(1.0),
            y: T::from(0.0),
        }
    }

    pub fn y_axis() -> Self {
        Self {
            x: T::from(0.0),
            y: T::from(1.0),
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().into().sqrt()
    }

    pub fn angle(&self, rhs: &Self) -> f32 {
        let dot = self.dot(rhs).into();
        let mag = self.magnitude() * rhs.magnitude();
        dot.acos() / mag
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag.into(),
            y: self.y / mag.into(),
        }
    }
}

impl<T: Numeric + Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vector {{ x: {:?}, y: {:?} }}",
            self.x, self.y
        )
    }
}

impl<T: Numeric + Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Self::new(self.x, self.y)
    }
}

impl<T: Numeric> Mul<Vector<T>> for Vector<T> {
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T: Numeric> Mul<&Vector<T>> for Vector<T> {
    type Output = T;
    fn mul(self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T: Numeric> Mul<Vector<T>> for &Vector<T> {
    type Output = T;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T: Numeric> Mul<&Vector<T>> for &Vector<T> {
    type Output = T;
    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

macro_rules! impl_vector_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op for Vector<T> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                }
            }
        }
        impl<T: Numeric> $op<&Vector<T>> for Vector<T> {
            type Output = Self;

            fn $method(self, rhs: &Self) -> Self::Output {
                Self {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                }
            }
        }
        impl<T: Numeric> $op<Vector<T>> for &Vector<T> {
            type Output = Vector<T>;

            fn $method(self, rhs: Vector<T>) -> Self::Output {
                Vector {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                }
            }
        }
        impl<T: Numeric> $op for &Vector<T> {
            type Output = Vector<T>;

            fn $method(self, rhs: &Vector<T>) -> Self::Output {
                Vector {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                }
            }
        }
        impl<T: Numeric> $op_assign for Vector<T> {
            fn $method_assign(&mut self, rhs: Self) {
                self.x = self.x.$method(rhs.x);
                self.y = self.y.$method(rhs.y);
            }
        }
        impl<T: Numeric> $op_assign<&Vector<T>> for Vector<T> {
            fn $method_assign(&mut self, rhs: &Self) {
                self.x = self.x.$method(rhs.x);
                self.y = self.y.$method(rhs.y);
            }
        }
    };
    () => {};
}
impl_vector_op!(Add, AddAssign, add, add_assign);
impl_vector_op!(Sub, SubAssign, sub, sub_assign);

macro_rules! impl_scalar_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op<T> for Vector<T> {
            type Output = Self;

            fn $method(self, scalar: T) -> Self::Output {
                Self {
                    x: self.x.$method(scalar),
                    y: self.y.$method(scalar),
                }
            }
        }
        impl<T: Numeric> $op<T> for &Vector<T> {
            type Output = Vector<T>;

            fn $method(self, scalar: T) -> Self::Output {
                Vector {
                    x: self.x.$method(scalar),
                    y: self.y.$method(scalar),
                }
            }
        }
        impl<T: Numeric> $op_assign<T> for Vector<T> {
            fn $method_assign(&mut self, scalar: T) {
                self.x = self.x.$method(scalar);
                self.y = self.y.$method(scalar);
            }
        }
    };
    () => {};
}
impl_scalar_op!(Mul, MulAssign, mul, mul_assign);
impl_scalar_op!(Div, DivAssign, div, div_assign);
