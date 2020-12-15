#[macro_use]
mod base;

#[macro_use]
mod int;

#[macro_use]
mod bit;

#[macro_use]
mod vcmp;


macro_rules! matrix_ops { ($M:expr, $N:expr, $W:ident) => (
	matrix_ops_base!($M, $N, $W);
	matrix_int!($M, $N, $W);
	matrix_bit!($M, $N, $W);
	matrix_vcmp!($M, $N, $W);
) }
