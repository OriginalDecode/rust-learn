
/*
	Copy and Clone are traits
*/

include!("macros.rs");

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

impl_add!(self, rhs, Vector3<T>, {
	Vector3::<T> 
	{
		x : self.x + rhs.x,
		y : self.y + rhs.y,
		z : self.z + rhs.z,
	}
});

impl_sub!(self, rhs, Vector3<T>, {
	Vector3::<T> 
	{
		x : self.x - rhs.x,
		y : self.y - rhs.y,
		z : self.z - rhs.z,
	}
});

impl_mul!(self, rhs, Vector3<T>, {
	Vector3::<T> 
	{
		x : self.x * rhs.x,
		y : self.y * rhs.y,
		z : self.z * rhs.z,
	}
});

impl_div!(self, rhs, Vector3<T>, {
	Vector3::<T> 
	{
		x : self.x / rhs.x,
		y : self.y / rhs.y,
		z : self.z / rhs.z,
	}
});

#[inline(always)]
pub fn vec3_dot<T>(a: Vector3<T>, b: Vector3<T>) -> T
	where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
	a.x * b.x + a.y * b.y + a.z * b.z
}


impl_add!(self, rhs, Vector4<T>, {
	Vector4::<T> 
	{
		x : self.x + rhs.x,
		y : self.y + rhs.y,
		z : self.z + rhs.z,
		w : self.w + rhs.w,
	}
});

impl_sub!(self, rhs, Vector4<T>, {
	Vector4::<T> 
	{
		x : self.x - rhs.x,
		y : self.y - rhs.y,
		z : self.z - rhs.z,
		w : self.w - rhs.w,
	}
});

impl_mul!(self, rhs, Vector4<T>, {
	Vector4::<T> 
	{
		x : self.x * rhs.x,
		y : self.y * rhs.y,
		z : self.z * rhs.z,
		w : self.w * rhs.w,
	}
});

impl_div!(self, rhs, Vector4<T>, {
	Vector4::<T> 
	{
		x : self.x / rhs.x,
		y : self.y / rhs.y,
		z : self.z / rhs.z,
		w : self.w / rhs.w,
	}
});


#[inline(always)]
pub fn vec4_dot<T>( a: Vector4<T>, b: Vector4<T> ) -> T
	where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
	a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}
