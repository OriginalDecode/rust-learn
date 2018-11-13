use std::ops;
pub struct Vector2<T>
{
    x: T,
    y: T,
}

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