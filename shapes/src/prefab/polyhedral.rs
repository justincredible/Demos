pub(super) trait Polyhedral {
    /// Required
    fn edges(&self) -> Vec<Vec<usize>>;

    /// Provided
    fn vertex_count(&self) -> usize {
        self.edges().len()
    }
}
