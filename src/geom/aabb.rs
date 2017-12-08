use std::f32::{INFINITY, NEG_INFINITY};
use ::cgmath::Vector3;
use ::cgmath::prelude::Zero;

/// An axis-aligned bounding box in 3D
pub struct Aabb {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>
}

impl Aabb {
    /// Creates the smallest aabb that encloses all of the points returned
    /// by the given iterator.
    /// Returns an aabb with max at negative infinity and min at positive infinity if
    /// the given iterator was empty.
    pub fn from_points<P>(points: P) -> Aabb
        where P: IntoIterator<Item = Vector3<f32>>
    {
        points.into_iter()
            .fold(
                Aabb {
                    min: Vector3::new(INFINITY, INFINITY, INFINITY),
                    max: Vector3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)
                },
                |Aabb { min, max }, p| {
                    let min_x = if p.x < min.x { p.x } else { min.x };
                    let min_y = if p.y < min.y { p.y } else { min.y };
                    let min_z = if p.z < min.z { p.z } else { min.z };

                    let max_x = if p.x > max.x { p.x } else { max.x };
                    let max_y = if p.y > max.y { p.y } else { max.y };
                    let max_z = if p.z > max.z { p.z } else { max.z };

                    Aabb {
                        min: Vector3::new(min_x, min_y, min_z),
                        max: Vector3::new(max_x, max_y, max_z)
                    }
                }
            )
    }

    /// Returns the smallest aabb that encloses all of the aabb in the given iterator.
    /// Returns an aabb with max at negative infinity and min at positive infinity if
    /// the given iterator was empty.
    pub fn union<A>(aabbs: A) -> Aabb
        where A: IntoIterator<Item = Aabb>
    {
        aabbs.into_iter()
            .fold(
                Aabb {
                    min: Vector3::new(INFINITY, INFINITY, INFINITY),
                    max: Vector3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)
                },
                |Aabb { min: acc_min, max: acc_max }, Aabb { min: aabb_min, max: aabb_max }| {
                    let min_x = if aabb_min.x < acc_min.x { aabb_min.x } else { acc_min.x };
                    let min_y = if aabb_min.y < acc_min.y { aabb_min.y } else { acc_min.y };
                    let min_z = if aabb_min.z < acc_min.z { aabb_min.z } else { acc_min.z };

                    let max_x = if aabb_max.x > acc_max.x { aabb_max.x } else { acc_max.x };
                    let max_y = if aabb_max.y > acc_max.y { aabb_max.y } else { acc_max.y };
                    let max_z = if aabb_max.z > acc_max.z { aabb_max.z } else { acc_max.z };

                    Aabb {
                        min: Vector3::new(min_x, min_y, min_z),
                        max: Vector3::new(max_x, max_y, max_z)
                    }
                }
            )
    }
}