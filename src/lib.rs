use std::fmt::Debug;
use std::ops::Neg;
use std::{
    array,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct VecXD<const X: usize, T>
where
    T: Add + Sub + Mul<T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    pub v: [T; X],
}

impl<const X: usize, T> VecXD<X, T>
where
    T: Add
        + Sub
        + Mul<T, Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + Default
        + Debug
        + Mul,
{
    pub fn new(data: [T; X]) -> Self {
        Self { v: data }
    }
    pub fn len_sq(&self) -> T
    where
        Self: Mul<Self, Output = T>,
    {
        *self * *self
    }
}

impl<const X: usize, T> Add<Self> for VecXD<X, T>
where
    T: Add<T, Output = T>
        + Sub
        + Mul<T, Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + Default
        + Debug,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] + rhs.v[n]);
        Self::new(x)
    }
}

impl<const X: usize, T> Sub for VecXD<X, T>
where
    T: Add
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + Default
        + Debug,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] - rhs.v[n]);
        Self::new(x)
    }
}

impl<const X: usize, T> Mul<T> for VecXD<X, T>
where
    T: Add + Sub + Mul<T, Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] * rhs);
        Self::new(x)
    }
}

impl<const X: usize, T> Mul<Self> for VecXD<X, T>
where
    T: Add + Sub + Mul<T, Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut x: T = Default::default();

        for i in 0..X {
            x += self.v[i] * rhs.v[i]
        }

        x
    }
}

impl<const X: usize, T> AddAssign for VecXD<X, T>
where
    T: Add + Sub + Mul<T, Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
    Self: Add<Self, Output = Self>,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const X: usize, T> SubAssign for VecXD<X, T>
where
    T: Add
        + Sub<Output = T>
        + Mul<T, Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + Default
        + Debug,
    Self: Sub<Self, Output = Self>,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const X: usize, T> MulAssign<T> for VecXD<X, T>
where
    T: Add + Sub + Mul<T, Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
    Self: Mul<T, Output = Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<const X: usize, T> Neg for VecXD<X, T>
where
    T: Add
        + Sub
        + Mul<T, Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + Default
        + Debug
        + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let x = array::from_fn(|i| -self.v[i]);
        VecXD::new(x)
    }
}
