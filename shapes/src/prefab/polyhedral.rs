use std::collections::HashSet;

use num_traits::{cast, Float, FloatConst, NumCast, one, Unsigned, zero};

use crate::shapes::{Configuration, Shape, Shaper};
use super::linear_algebra::oriented_plane;

pub(super) trait Polyhedral {
    fn vertices<C>(&self) -> Vec<[C; 3]>
    where C: Float + FloatConst;

    fn edges(&self) -> Vec<Vec<usize>>;

    fn vertices_per_face(&self) -> usize;

    fn vertex_count(&self) -> usize {
        self.edges().len()
    }

    fn faces(&self) -> HashSet<Vec<usize>> {
        platonic_solid(self)
    }

    fn strips(&self) -> Option<Vec<Vec<usize>>> {
        None
    }
}

pub(super) fn platonic_solid(solid: &(impl Polyhedral + ?Sized)) -> HashSet<Vec<usize>> {
    let mut faces = HashSet::new();
    let edges = solid.edges();

    for i in 0..solid.vertex_count() {
        find_face(&mut faces, &edges, solid.vertices_per_face(), vec![i]);
    }

    faces
}

fn find_face(faces: &mut HashSet<Vec<usize>>, edges: &Vec<Vec<usize>>, target: usize, mut current: Vec<usize>) {
    let previous = current[current.len()-1];
    if current.len() == target {
        if edges[previous].contains(&current[0]) {
            current.sort();
            faces.insert(current);
        }
    } else if current.len() < target {
        for i in edges[previous].iter().map(Clone::clone).filter(|x| !current.contains(x)) {
            let mut next = current.clone();
            next.push(i);
            find_face(faces, edges, target, next);
        }
    }
}

impl<T, C, I> Shaper<C, I> for T
where
    T: Polyhedral,
    C: Float + FloatConst,
    I: Copy + NumCast + Unsigned,
{
    fn make(&self, request: Configuration) -> Shape<C, I> {
        let mut vertices: Vec<[C; 3]> = self.vertices();

        if request.orientation.is_left() {
            for vertex in &mut vertices {
                vertex[2] = vertex[2].neg();
            }
        }

        let i = vec![zero(), one()]
            .into_iter()
            .chain((2..vertices.len()).map(|i| cast::<_, I>(i).unwrap()))
            .collect::<Vec<_>>();

        if request.prefer_strips && let Some(index_strips) = self.strips() {
            let lookup = |idx| if request.orientation.is_ccw() {
                i[idx]
            } else {
                i[self.vertex_count() - 1 - idx]
            };
            let mut strips = vec![];
            for strip in index_strips {
                strips.push(strip.into_iter().map(lookup).collect());
            }

            Shape::Strips { vertices, strips }
        } else {
            let mut normals = vec![];
            let mut indices = vec![];
            for face in self.faces() {
                let (normal, triangle) = oriented_plane(&vertices, &face, request.orientation);
                let mut iterator: Vec<_> = triangle.into();
                for i in 0..self.vertices_per_face() - 3 {
                    iterator = iterator.into_iter()
                        .chain(oriented_plane(&vertices, &face[i+1..], request.orientation).1)
                        .collect();
                }
                for index in iterator {
                    indices.push(i[index]);
                }
                normals.push(normal);
            }

            if request.generate_normals {
                Shape::NormalTriangles { vertices, normals, indices }
            } else {
                Shape::Triangles { vertices, indices }
            }
        }
    }
}
