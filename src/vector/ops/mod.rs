#[macro_use]
mod base;

#[macro_use]
mod int;

#[macro_use]
mod bit;

#[macro_use]
mod vcmp;


macro_rules! vector_ops { ($N:expr, $V:ident) => (
	vector_ops_base!($N, $V);
	vector_int!($N, $V);
	vector_bit!($N, $V);
	vector_vcmp!($N, $V);
) }
