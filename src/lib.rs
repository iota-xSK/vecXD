use std::fmt::Debug;
use std::{
    array,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct VecXD<const X: usize, T>
where
    T: Add + Sub + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    pub v: [T; X],
}

impl<const X: usize, T> VecXD<X, T>
where
    T: Add + Sub + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    pub fn new(data: [T; X]) -> Self {
        Self { v: data }
    }
}

impl<const X: usize, T> Add for VecXD<X, T>
where
    T: Add<Output = T> + Sub + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] + rhs.v[n]);
        Self::new(x)
    }
}

impl<const X: usize, T> Sub for VecXD<X, T>
where
    T: Add + Sub<Output = T> + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] - rhs.v[n]);
        Self::new(x)
    }
}

impl<const X: usize, T> Mul<T> for VecXD<X, T>
where
    T: Add + Sub + Mul<Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let x: [T; X] = array::from_fn(|n| self.v[n] * rhs);
        Self::new(x)
    }
}

impl<const X: usize, T> Mul<Self> for VecXD<X, T>
where
    T: Add + Sub + Mul<Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
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
    T: Add<Output = T> + Sub + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const X: usize, T> SubAssign for VecXD<X, T>
where
    T: Add + Sub<Output = T> + Mul + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const X: usize, T> MulAssign<T> for VecXD<X, T>
where
    T: Add + Sub + Mul<Output = T> + AddAssign + SubAssign + MulAssign + Copy + Default + Debug,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}
