use std::collections::HashSet;

pub(super) trait Polyhedral {
    fn edges(&self) -> Vec<Vec<usize>>;

    fn vertices_per_face(&self) -> usize;

    fn vertex_count(&self) -> usize {
        self.edges().len()
    }

    fn faces(&self) -> HashSet<Vec<usize>> {
        platonic_solid(self)
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
    let last = current[current.len()-1];
    if current.len() == target {
        if edges[last].contains(&current[0]) {
            current.sort();
            faces.insert(current);
        }
    } else if current.len() < target {
        for i in edges[last].iter().map(Clone::clone).filter(|x| !current.contains(x)) {
            let mut next = current.clone();
            next.push(i);
            find_face(faces, edges, target, next);
        }
    }
}
