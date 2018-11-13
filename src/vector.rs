#[derive(Copy, Clone)]
pub struct Vector2<T>
{
    x: T,
    y: T,
}

use std::ops;
impl<T> ops::Add<Vector2<T>> for Vector2<T> 
where T: ops::Add<T, Output=T>
{
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T> ) -> Vector2<T> {
        Vector2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Sub<Vector2<T>> for Vector2<T> 
where T: ops::Sub<T, Output=T>
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T> ) -> Vector2<T> {
        Vector2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::Mul<Vector2<T>> for Vector2<T> 
where T: ops::Mul<T, Output=T>
{
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T> ) -> Vector2<T> {
        Vector2::<T> {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

