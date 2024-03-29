use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

// A Vector 2D with generic type T
#[derive(Default, Debug, Clone, Copy)]
struct Vec2<T>
where
    T: Default + Debug,
{
    x: T,
    y: T,
}

impl<T> Display for Vec2<T>
where
    T: Default + Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl<T> Add for Vec2<T>
where
    T: Default + Debug + Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Default + Debug + Clone + Copy + Mul<Output = T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

trait ToVec2<T>
where
    T: Default + Debug,
{
    fn to_vec2(&self) -> Vec2<T>;
}

impl ToVec2<usize> for usize {
    fn to_vec2(&self) -> Vec2<usize> {
        Vec2 { x: *self, y: *self }
    }
}

fn main() {
    println!("Testing Vec2<T> with T: String");
    let vec_str = Vec2 {
        x: "Holi!",
        y: "Chau!",
    };
    println!("{vec_str:?}");
    println!("{vec_str}\n");

    println!("Testing Default Vec2<T> with T: usize");
    let vec1: Vec2<usize> = Vec2::default();
    println!("{vec1:?}");
    println!("{vec1}\n");

    println!("Testing ToVec2 trait with usize");
    let vec2 = 10.to_vec2();
    println!("{vec2:?}");
    println!("{vec2}\n");

    println!("Testing Add trait with Vec2<T> where T: Add");
    let vec_sum = vec1 + vec2;
    println!("{vec1:?} + {vec2:?} = {vec_sum:?}");
    println!("{vec1} + {vec2} = {vec_sum}\n");

    println!("Testing Mul trait with Vec2<T> where T: Mul");
    let vec_mul = vec_sum * 2;
    println!("{vec_sum:?} * 2 = {vec_mul:?}");
    println!("{vec_sum} * 2 = {vec_mul}\n");
}
