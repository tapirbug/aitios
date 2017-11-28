
use std::io;
use ::cgmath::Vector3;
use ::rand;
use ::geom::tri::area;

/// Represents the surface of a mesh as a point-based model
pub struct Surface {
    pub samples: Vec<Surfel>
}

/// Represents an element of the surface of an object
pub struct Surfel {
    position: Vector3<f32>,
    /// Deterioration rate of the probability of a gammaton moving further in a straight line
    delta_straight: f32,
    /// Deterioration rate of the probability of a gammaton moving in a piecewise approximated parabolic path
    delta_parabolic: f32,
    /// Deterioration rate of the probability of a gammaton flowing in a tangent direction
    delta_flow: f32,
    /// Holds the amount of materials as numbers in the interval 0..1
    materials: Vec<f32>
}

impl Surface {
    /// Creates a surface model by sampling an amount of random points on each
    /// of the traingles in the given indexed mesh that is proportional to the
    /// area of the individual triangles. This way, the sampling is sort of uniform
    /// but not really.
    ///
    /// The initial values of the surfels are provided as parameters to this function.
    pub fn from_triangles(positions: &Vec<f32>, indices: &Vec<u32>, delta_straight: f32, delta_parabolic: f32, delta_flow: f32, initial_material_composition: &Vec<f32>) -> Surface
    {
        // Collect 3-tuples of Vector3 representing the vertices of each indexed triangle in the mesh
        let triangles = indices.chunks(3)
                               .map(|i|
                                (
                                    Vector3::new(positions[(3*i[0]+0) as usize], positions[(3*i[0]+1) as usize], positions[(3*i[0]+2) as usize]),
                                    Vector3::new(positions[(3*i[1]+0) as usize], positions[(3*i[1]+1) as usize], positions[(3*i[1]+2) as usize]),
                                    Vector3::new(positions[(3*i[2]+0) as usize], positions[(3*i[2]+1) as usize], positions[(3*i[2]+2) as usize])
                                )
                               );

        let surface_points = triangles.fold(
            Vec::<Vector3<f32>>::new(),
            |mut acc, (v0, v1, v2)| {
                let surfels_per_sqr_unit = 1000.0;
                let surfel_count = (surfels_per_sqr_unit * area(v0, v1, v2)).ceil() as i32;

                for _ in 0..surfel_count {
                    let u = rand::random::<f32>();
                    let v = rand::random::<f32>();
                    let random_point = (1.0 - u.sqrt()) * v0 +
                                       (u.sqrt() * (1.0 - v)) * v1 +
                                       (u.sqrt() * v) * v2;

                    acc.push(random_point);
                }

                acc
            }
        );

        Surface::from_points(surface_points, delta_straight, delta_parabolic, delta_flow, initial_material_composition)
    }

    pub fn from_points<P>(points: P, delta_straight: f32, delta_parabolic: f32, delta_flow: f32, initial_material_composition: &Vec<f32>) -> Surface
    where
        P : IntoIterator<Item = Vector3<f32>> {

        Surface {
            samples: points.into_iter()
                .map(
                    |position| Surfel {
                        position,
                        delta_straight,
                        delta_parabolic,
                        delta_flow,
                        materials: initial_material_composition.clone()
                    }
                )
                .collect()
        }
    }

    pub fn dump<S : io::Write>(&self, sink: &mut S) -> io::Result<usize> {
        let mut written : usize = 0;

        written += sink.write("# Surface Model\n".as_bytes())?;
        written += sink.write("# Generated by surf.rs\n\n".as_bytes())?;

        written += sink.write("g surface\n\n".as_bytes())?;

        for &point in self.samples.iter().map(|s| &s.position) {
            // Write all the points as vertices
            let vertex_line = format!("v {} {} {}\n", point.x, point.y, point.z);
            written += sink.write(vertex_line.as_bytes())?;
        }

        written += sink.write("\n".as_bytes())?;

        // OBJ indices are 1-based, hence +1
        for idx in (0+1)..(self.samples.len()+1) {
            // Write points as 1-dimensional faces
            let face_line = format!("f {}\n", idx);
            written += sink.write(face_line.as_bytes())?;
        }

        Ok(written)
    }
}
