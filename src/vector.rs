
// use std::ops::{ Add, Mul, Neg, Sub, Div };
pub type Vector2<T> =[T;2];

impl<T> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, _rhs: Vector2<T> ) -> Vector2<T> {
        Vector2([self[0] + _rhs[0], self[1] + _rhs[1]])
    }
}

// impl<T> Vector2<T> {
//     fn new( _x : T, _y : T) -> Vector2<T> {
//         Vector2<T> { x : _x, y : _y }
//     }
// }

// impl<T> std::ops::Add<Vector2<T>> for Vector2<T> {
//     type Output = Vector2<T>;
//     fn add(self, _rhs: Vector2<T>) -> Vector2<T> {
//         return Vector2 { x : self.x + _rhs.x, y : self.y + _rhs.y };
//     }
// }