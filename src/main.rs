include!("vector.rs");

fn main() {
    println!("Hello, world!");

    let _vec20 = Vector2{x:31.0, y:13.0};
    let _vec21 = Vector2{x:42.0, y:13.0};

    let _vec22 = _vec20 + _vec21;

    println!("{}, {}", _vec22.x, _vec22.y);

}