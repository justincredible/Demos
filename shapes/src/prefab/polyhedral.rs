pub(super) trait Polyhedral {
    fn vertex_count(&self) -> usize;

    fn edges(&self) -> Vec<Vec<usize>> {
        Vec::with_capacity(0)
    }
}
