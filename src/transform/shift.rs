#[cfg(feature = "rand")]
use crate::distr::Normal;
use crate::{Transform, Vector, transform::Directional};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, AbsDiffEq};
use core::ops::Neg;
use num_traits::{Num, Zero};
#[cfg(feature = "rand")]
use rand_::{distributions::Distribution, Rng};

/// Shift transformation.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Shift<T, const N: usize> {
    pos: Vector<T, N>,
}

pub type Shift2<T> = Shift<T, 2>;
pub type Shift3<T> = Shift<T, 3>;
pub type Shift4<T> = Shift<T, 4>;

impl<T, const N: usize> Shift<T, N> {
    pub fn from_vector(pos: Vector<T, N>) -> Self {
        Self { pos }
    }
    pub fn into_vector(self) -> Vector<T, N> {
        self.pos
    }
}
impl<T, const N: usize> From<Vector<T, N>> for Shift<T, N> {
    fn from(pos: Vector<T, N>) -> Self {
        Self::from_vector(pos)
    }
}
impl<T, const N: usize> From<Shift<T, N>> for Vector<T, N> {
    fn from(shift: Shift<T, N>) -> Self {
        shift.into_vector()
    }
}

impl<T, const N: usize> Transform<Vector<T, N>> for Shift<T, N>
where
    T: Neg<Output = T> + Num + Copy,
{
    fn identity() -> Self {
        Self {
            pos: Vector::zero(),
        }
    }
    fn inv(self) -> Self {
        Self { pos: -self.pos }
    }
    fn apply(&self, pos: Vector<T, N>) -> Vector<T, N> {
        pos + self.pos
    }
    fn deriv(&self, _pos: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        dir
    }
    fn chain(self, other: Self) -> Self {
        Self {
            pos: self.pos + other.pos,
        }
    }
}

impl<T, const N: usize> Directional<Vector<T, N>> for Shift<T, N>
where
    Self: Transform<Vector<T, N>>
{
    fn apply_dir(&self, _: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        dir
    }
    fn apply_normal(&self, _: Vector<T, N>, normal: Vector<T, N>) -> Vector<T, N> {
        normal
    }
}

#[cfg(feature = "rand")]
impl<T, const N: usize> Distribution<Shift<T, N>> for Normal
where
    Normal: Distribution<Vector<T, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shift<T, N> {
        Shift::from(self.sample(rng))
    }
}

#[cfg(feature = "approx")]
impl<T, const N: usize> AbsDiffEq for Shift<T, N>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.pos, other.pos, epsilon = epsilon)
    }
}
