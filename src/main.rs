include!("vector.rs");

fn main() {
    println!("Hello, world!");

    let _vec20 = Vector2{x:31.0, y:13.0};
    let _vec21 = Vector2{x:42.0, y:13.0};

    let add_vec22 = _vec20 + _vec21;
    let sub_vec22 = _vec20 - _vec21;
    let mul_vec22 = _vec20 * _vec21;

    println!("{}, {}", add_vec22.x, add_vec22.y);
    println!("{}, {}", sub_vec22.x, sub_vec22.y);
    println!("{}, {}", mul_vec22.x, mul_vec22.y);

}