use cgmath::{Vector3, Point3};

use util::types::Float;

pub trait AABB {
    fn get_min(&self) -> Point3<Float>;
    fn get_max(&self) -> Point3<Float>;
}

#[derive(Copy, Clone)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

fn clip_ray<T>(axis: Axis, ray_start: &Point3<Float>, ray_stop: &Point3<Float>, aabb_box: &T, aabb_box_pos: &Point3<Float>, f_low: Float, f_high: Float) -> Option<(Float, Float)> 
    where T: AABB 
{
    let axis = axis as usize;
    let mut f_dim_low = (aabb_box.get_min()[axis] + aabb_box_pos[axis] - ray_start[axis]) / (ray_stop[axis] - ray_start[axis]);
    let mut f_dim_high = (aabb_box.get_max()[axis] + aabb_box_pos[axis] - ray_start[axis]) / (ray_stop[axis] - ray_start[axis]);

    if f_dim_high < f_dim_low {
        let tmp  = f_dim_high;
        f_dim_high = f_dim_low;
        f_dim_low = tmp;
    }

    if f_dim_high < f_low {
        None
    } else if f_dim_low > f_high {
        None
    } else {
        let f_low = f_dim_low.max(f_low);
        let f_high = f_dim_high.min(f_high);
        if f_low > f_high {
            None
        } else {
            Some((f_low, f_high))
        }
    }
}

pub fn ray_intersect<T>(ray_pos: &Point3<Float>, ray_dir: &Vector3<Float>, max_ray_len: Float, aabb_box: &T, aabb_box_pos: &Point3<Float>) -> Option<(Float, Point3<Float>)>
    where T: AABB
{
    let end = ray_pos + (ray_dir * max_ray_len);
    let mut f_low = 0.0;
    let mut f_high = 1.0;
    
    for &axis in [Axis::X, Axis::Y, Axis::Z].iter() {
        let result = clip_ray(axis, ray_pos, &end, aabb_box, aabb_box_pos, f_low, f_high);
        if let Some((near, far)) = result {
            f_low = near;
            f_high = far;
        } else {
            return None;
        }
    }

    Some((f_low, ray_pos + (end - ray_pos) * f_low))
}

