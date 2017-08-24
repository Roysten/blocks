use cgmath::Point3;

use util::types::Float;

pub fn clamp<T>(v: T, min: T, max: T) -> T 
    where T: PartialOrd
{
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

pub fn safe_cast(a: &Point3<Float>) -> Point3<usize> {
    Point3::new(
        a.x.max(0.0) as usize,
        a.y.max(0.0) as usize,
        a.z.max(0.0) as usize,
    )
}
