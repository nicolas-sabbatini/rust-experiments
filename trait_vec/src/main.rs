use std::{
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(Default, Debug)]
struct Vec2<T>
where
    T: Default + Display + Debug + Add,
{
    x: T,
    y: T,
}

impl<T> Display for Vec2<T>
where
    T: Default + Display + Debug + Add,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

trait ToVec2<T>
where
    T: Default + Display + Debug + Add,
{
    fn to_vec2(&self) -> Vec2<T>;
}

impl ToVec2<usize> for usize {
    fn to_vec2(&self) -> Vec2<usize> {
        Vec2 { x: *self, y: *self }
    }
}

fn main() {
    let vec1: Vec2<usize> = Vec2::default();

    println!("{vec1:?}");
    println!("{vec1}");

    // let vec_str = Vec2 {
    //     x: "Holi!",
    //     y: "Chau!",
    // };
    //
    // println!("{vec_str:?}");
    // println!("{vec_str}");

    let vec2 = 10.to_vec2();

    println!("{vec2:?}");
    println!("{vec2}");
}
