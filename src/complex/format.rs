use crate::complex::{Complex, Quaternion};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

impl<T: Debug> Debug for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Complex({:?}, {:?})", self.re_ref(), self.im_ref(),)
    }
}
impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Complex({}, {})", self.re_ref(), self.im_ref(),)
    }
}

impl<T: Debug> Debug for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Quaternion({:?}, {:?}, {:?}, {:?})",
            self.w_ref(),
            self.x_ref(),
            self.y_ref(),
            self.z_ref(),
        )
    }
}
impl<T: Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.w_ref(),
            self.x_ref(),
            self.y_ref(),
            self.z_ref(),
        )
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use std::format;

    #[test]
    fn quaternion() {
        let q = Quaternion::<i32>::new(1, -2, 3, -4);
        assert_eq!(format!("{:?}", q), "Quaternion(1, -2, 3, -4)");
        assert_eq!(format!("{}", q), "Quaternion(1, -2, 3, -4)");
    }
}
