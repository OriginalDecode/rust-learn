
/*
	Copy and Clone are traits
*/
#[derive(Copy, Clone)]
pub struct Vector2<T>
{
	x: T,
	y: T,
}

#[derive(Copy, Clone)]
pub struct Vector3<T>
{
	x: T,
	y: T,
	z: T,
}

#[derive(Copy, Clone)]
pub struct Vector4<T>
{
	x: T,
	y: T,
	z: T,
	w: T,
}

macro_rules! impl_op {
	($this:ident, $rhs:ident, $vector:ty, $body:block, $fname:ident, $trait:ident) => {
		impl<T> $trait<$vector> for $vector 
			where T: $trait<T, Output=T>
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


use std::ops:: { Add, Mul,  Sub, Div };

impl_add!(self, rhs, Vector2<T>, {
	Vector2::<T> 
	{
		x : self.x + rhs.x,
		y : self.y + rhs.y,
	}
});

impl_sub!(self, rhs, Vector2<T>, {
	Vector2::<T> 
	{
		x: self.x - rhs.x,
		y: self.y - rhs.y,
	}
});

impl_mul!(self, rhs, Vector2<T>, {
	Vector2::<T> 
	{
		x: self.x * rhs.x,
		y: self.y * rhs.y,
	}
});

impl_div!(self, rhs, Vector2<T>, {
	Vector2::<T> 
	{
		x: self.x / rhs.x,
		y: self.y / rhs.y,
	}
});

#[inline(always)]
pub fn vec2_dot<T>( a : Vector2<T>, b : Vector2<T>) -> T 
	where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
	a.x * b.x + a.y * b.y
}

#[inline(always)]
pub fn vec3_dot<T>(a: Vector3<T>, b: Vector3<T>) -> T
	where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
	a.x * b.x + a.y * b.y + a.z * b.z
}

#[inline(always)]
pub fn vec4_dot<T>( a: Vector4<T>, b: Vector4<T> ) -> T
	where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
	a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}
