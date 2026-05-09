pub(super) trait Polyhedral {
    fn edges(&self) -> Vec<Vec<usize>>;

    fn vertex_count(&self) -> usize {
        self.edges().len()
    }
}
