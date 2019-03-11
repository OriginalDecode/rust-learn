
macro_rules! impl_op {
	($this:ident, $rhs:ident, $vector:ty, $body:block, $fname:ident, $trait:ident) => {
		impl<T> $trait<$vector> for $vector 
			where T: Copy + $trait<T, Output=T>
		{
			type Output = $vector;
			fn $fname($this, $rhs : $vector ) -> $vector {
				$body
			}
		}
	};
}

macro_rules! impl_add {
	($this:ident, $rhs:ident, $vector:ty, $body:block) => {
		impl_op!($this, $rhs, $vector, $body, add, Add);
	};
}

macro_rules! impl_sub {
	($this:ident, $rhs:ident, $vector:ty, $body:block) => {
		impl_op!($this, $rhs, $vector, $body, sub, Sub);
	};
}

macro_rules! impl_mul {
	($this:ident, $rhs:ident, $vector:ty, $body:block) => {
		impl_op!($this, $rhs, $vector, $body, mul, Mul);
	};
}

macro_rules! impl_div {
	($this:ident, $rhs:ident, $vector:ty, $body:block) => {
		impl_op!($this, $rhs, $vector, $body, div, Div);
	};
}
