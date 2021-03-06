#[cfg(feature = "rand")]
use crate::distr::{Invertible, Normal};
use crate::{
    traits::{Dot, Normalize},
    transform::{Reorder, Shift, Directional},
    Matrix, Transform, Vector,
};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, AbsDiffEq};
use core::ops::Neg;
use num_traits::{Float, Num, NumCast, One, Inv};
#[cfg(feature = "rand")]
use rand_::{distributions::Distribution, Rng};

/// Linear transformation.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Linear<T, const N: usize> {
    lin: Matrix<T, N, N>,
}

pub type Linear2<T> = Linear<T, 2>;
pub type Linear3<T> = Linear<T, 3>;
pub type Linear4<T> = Linear<T, 4>;

impl<T, const N: usize> Linear<T, N> {
    pub fn from_matrix(lin: Matrix<T, N, N>) -> Self {
        Self { lin }
    }
    pub fn into_matrix(self) -> Matrix<T, N, N> {
        self.lin
    }
}
impl<T, const N: usize> From<Matrix<T, N, N>> for Linear<T, N> {
    fn from(lin: Matrix<T, N, N>) -> Self {
        Self::from_matrix(lin)
    }
}
impl<T, const N: usize> From<Linear<T, N>> for Matrix<T, N, N> {
    fn from(lin: Linear<T, N>) -> Matrix<T, N, N> {
        lin.into_matrix()
    }
}

impl<T, const N: usize> Transform<Vector<T, N>> for Linear<T, N>
where
    T: Neg<Output = T> + Num + Copy,
{
    fn identity() -> Self {
        Self { lin: Matrix::one() }
    }
    fn inv(self) -> Self {
        Self {
            lin: self.lin.inv(),
        }
    }
    fn apply(&self, pos: Vector<T, N>) -> Vector<T, N> {
        self.lin.dot(pos)
    }
    fn deriv(&self, _pos: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        self.apply(dir)
    }
    fn chain(self, other: Self) -> Self {
        Self {
            lin: self.lin.dot(other.lin),
        }
    }
}

impl<T, const N: usize> Linear<T, N>
where
    T: Neg<Output = T> + Num + Copy,
    Matrix<T, N, N>: Inv<Output=Matrix<T, N, N>>,
{
    pub fn normal_transform(self) -> Self {
        Self { lin: self.lin.inv().transpose() }
    }
}

impl<T, const N: usize> Directional<Vector<T, N>> for Linear<T, N>
where
    Self: Transform<Vector<T, N>>,
    T: Neg<Output = T> + Num + Copy,
    Vector<T, N>: Normalize,
    Matrix<T, N, N>: Inv<Output=Matrix<T, N, N>>,
{
    fn apply_dir(&self, pos: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        self.deriv(pos, dir).normalize()
    }
    fn apply_normal(&self, _: Vector<T, N>, normal: Vector<T, N>) -> Vector<T, N> {
        self.normal_transform().apply(normal).normalize()
    }
}

#[cfg(feature = "rand")]
impl<T, const N: usize> Distribution<Linear<T, N>> for Normal
where
    Normal: Distribution<Matrix<T, N, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Linear<T, N> {
        Linear::from_matrix(self.sample(rng))
    }
}
#[cfg(feature = "rand")]
impl<T, const N: usize> Distribution<Linear<T, N>> for Invertible
where
    Invertible: Distribution<Matrix<T, N, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Linear<T, N> {
        Linear::from_matrix(self.sample(rng))
    }
}

#[cfg(feature = "approx")]
impl<T, const N: usize> AbsDiffEq for Linear<T, N>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.lin, other.lin, epsilon = epsilon)
    }
}

impl<T> Linear<T, 3>
where
    T: Float,
{
    /// Returns the transformation that rotates `-z`-axis to `dir`
    /// and `y`-axis to `up`.
    pub fn look_at(dir: Vector<T, 3>, up: Vector<T, 3>) -> Self {
        let right = dir.cross(up).normalize();
        let strict_up = right.cross(dir);
        Self::from(Matrix::from([right, strict_up, -dir]).transpose())
    }
}
impl<T> Linear<T, 3>
where
    T: Float + NumCast,
{
    /// Returns any of transformations that rotate `dir` to `-z`-axis.
    pub fn look_at_any(dir: Vector<T, 3>) -> Self {
        if dir.z().abs() < T::from(0.5).unwrap() {
            Self::look_at(dir, Vector::from([T::zero(), T::zero(), T::one()]))
        } else {
            Self::look_at(dir, Vector::from([T::zero(), T::one(), T::zero()]))
        }
    }
}

impl<T, const N: usize> Reorder<Linear<T, N>, Vector<T, N>> for Shift<T, N>
where
    Linear<T, N>: Transform<Vector<T, N>> + Copy,
    Self: Transform<Vector<T, N>>,
{
    fn reorder(self, other: Linear<T, N>) -> (Linear<T, N>, Shift<T, N>) {
        (other, other.inv().apply(self.into_vector()).into())
    }
}
impl<T, const N: usize> Reorder<Shift<T, N>, Vector<T, N>> for Linear<T, N>
where
    Self: Transform<Vector<T, N>>,
    Shift<T, N>: Transform<Vector<T, N>>,
{
    fn reorder(self, other: Shift<T, N>) -> (Shift<T, N>, Linear<T, N>) {
        (self.apply(other.into_vector()).into(), self)
    }
}

#[cfg(all(test, feature = "rand", feature = "approx"))]
mod tests {
    use super::*;
    use crate::distr::{Normal, Unit};
    use approx::assert_abs_diff_eq;
    use rand_::prelude::*;
    use rand_xorshift::XorShiftRng;

    const SAMPLE_ATTEMPTS: usize = 256;

    #[test]
    fn linearity() {
        const EPS: f64 = 1e-14;
        let mut rng = XorShiftRng::seed_from_u64(0xBEE);

        for _ in 0..SAMPLE_ATTEMPTS {
            let m: Matrix<f64, 3, 3> = rng.sample(&Normal);
            let x: Vector<f64, 3> = rng.sample(&Normal);
            let a: f64 = rng.sample(&Normal);

            assert_abs_diff_eq!(
                Linear::from(m * a).apply(x),
                Linear::from(m).apply(x * a),
                epsilon = EPS
            );
            assert_abs_diff_eq!(
                Linear::from(m * a).apply(x),
                Linear::from(m).apply(x) * a,
                epsilon = EPS
            );
        }
    }

    #[test]
    fn chaining() {
        const EPS: f64 = 1e-14;
        let mut rng = XorShiftRng::seed_from_u64(0xBEA);

        for _ in 0..SAMPLE_ATTEMPTS {
            let a: Linear<f64, 3> = rng.sample(&Normal);
            let b: Linear<f64, 3> = rng.sample(&Normal);
            let c: Vector<f64, 3> = rng.sample(&Normal);

            assert_abs_diff_eq!(a.chain(Linear::identity()), a, epsilon = EPS);
            assert_abs_diff_eq!(Linear::identity().chain(b), b, epsilon = EPS);
            assert_abs_diff_eq!(a.chain(b).apply(c), a.apply(b.apply(c)), epsilon = EPS);
        }
    }

    #[test]
    fn inversion() {
        const EPS: f64 = 1e-12;
        let mut rng = XorShiftRng::seed_from_u64(0xBEB);

        for _ in 0..SAMPLE_ATTEMPTS {
            let a: Linear<f64, 3> = rng.sample(&Invertible);
            let x: Vector<f64, 3> = rng.sample(&Normal);

            assert_abs_diff_eq!(a.chain(a.inv()), Linear::identity(), epsilon = EPS);
            assert_abs_diff_eq!(a.inv().chain(a), Linear::identity(), epsilon = EPS);
            assert_abs_diff_eq!(a.inv().apply(a.apply(x)), x, epsilon = EPS);
            assert_abs_diff_eq!(a.apply(a.inv().apply(x)), x, epsilon = EPS);
        }
    }

    #[test]
    fn look_to_the_direction() {
        const EPS: f64 = 1e-14;
        let mut rng = XorShiftRng::seed_from_u64(0xBEC);

        for _ in 0..SAMPLE_ATTEMPTS {
            let d: Vector<f64, 3> = rng.sample(&Unit);
            let m = Linear::look_at_any(d);

            assert_abs_diff_eq!(m.apply(Vector::from([0.0, 0.0, -1.0])), d, epsilon = EPS);
        }
    }
}
