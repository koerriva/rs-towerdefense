pub mod physics;

pub struct Point{
    x:f32,
    y:f32,
    z:f32,
    old_x:f32,
    old_y:f32,
    old_z:f32
}

pub struct Stick{
    p0:&Point,
    p1:&Point
}
