use vec2d::{Vec2d, Vec2dMath};

mod vec2d;

fn main() {
    let v1 = vec2d::Vec2d::new(4.0, 3.0);

    println!("Vector: {}", v1);
    println!("Vector {} norm: {}", v1, v1.norm());
    println!("Unit vector: {}", vec2d::Vec2d::get_unit_vec());

    let v2 = vec2d::Vec2d::new(4.0, 3.0);
    let v3 = vec2d::Vec2d::new(5.0, 3.0);

    println!("{} == {}: {}", v1, v2, v1 == v2);
    println!("{} == {}: {}", v1, v3, v1 == v3);
    println!("{} + {} = {}", v1, v3, &v1 + &v3);
    println!("{} - {} = {}", v1, v3, &v1 - &v3);

    let mut v1 = v1;
    v1.scale(2.0);

    println!("{v1}");
    println!("Structure method: {}", v1.scalar_prod(&v2));
    println!("Trait method: {}", <Vec2d as Vec2dMath>::scalar_prod(&v1, &v2));
}
