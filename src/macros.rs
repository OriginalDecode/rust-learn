
macro_rules! impl_op {
	($this:ident, $rhs:ident, $type:ty, $body:block, $fname:ident, $trait:ident) => {
		impl<T> $trait<$type> for $type 
			where T: Copy + $trait<T, Output=T>
		{
			type Output = $type;
			fn $fname($this, $rhs : $type ) -> $type {
				$body
			}
		}
	};
}

macro_rules! impl_add {
	($this:ident, $rhs:ident, $type:ty, $body:block) => {
		impl_op!($this, $rhs, $type, $body, add, Add);
	};
}

macro_rules! impl_sub {
	($this:ident, $rhs:ident, $type:ty, $body:block) => {
		impl_op!($this, $rhs, $type, $body, sub, Sub);
	};
}

macro_rules! impl_mul {
	($this:ident, $rhs:ident, $type:ty, $body:block) => {
		impl_op!($this, $rhs, $type, $body, mul, Mul);
	};
}

macro_rules! impl_div {
	($this:ident, $rhs:ident, $type:ty, $body:block) => {
		impl_op!($this, $rhs, $type, $body, div, Div);
	};
}
