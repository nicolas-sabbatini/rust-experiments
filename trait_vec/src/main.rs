use std::fmt::Display;

#[derive(Default, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

fn main() {
    let vec1: Vec2<usize> = Vec2::default();

    println!("{vec1:?}");
    println!("{vec1}");

    let vec_str = Vec2 {
        x: "Holi!",
        y: "Chau!",
    };

    println!("{vec_str:?}");
    println!("{vec_str}");
}
