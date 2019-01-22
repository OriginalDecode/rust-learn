
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
	( $trait:ident, $vector:ty, $_block:block ) => {
		impl<T> $trait<$vector> for $vector 
			where T: $trait<T, Output=T> {
				$_block
			}
	};
}

use std::ops:: { Add, Mul,  Sub, Div };

// impl<T> Add<Vector2<T>> for Vector2<T> 
// 	where T: Add<T, Output=T>
impl_op! { Add, Vector2<T>, {
	type Output = Vector2<T>;
	fn add(self, rhs: Vector2<T> ) -> Vector2<T> 
	{
		Vector2::<T> 
		{
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}}

impl<T> Sub<Vector2<T>> for Vector2<T> 
	where T: Sub<T, Output=T>
{
	type Output = Vector2<T>;
	fn sub(self, rhs: Vector2<T> ) -> Vector2<T> 
	{
		Vector2::<T> 
		{
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Mul<Vector2<T>> for Vector2<T> 
	where T: Mul<T, Output=T>
{
	type Output = Vector2<T>;
	fn mul(self, rhs: Vector2<T> ) -> Vector2<T> 
	{
		Vector2::<T> 
		{
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl<T> Div<Vector2<T>> for Vector2<T>
	where T: Div<T, Output=T>
{
	type Output = Vector2<T>;
	fn div(self, rhs: Vector2<T>) ->Vector2<T> 
	{
		Vector2::<T> 
		{
			x:self.x / rhs.x,
			y:self.y / rhs.y, 
		}
	}
}

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
