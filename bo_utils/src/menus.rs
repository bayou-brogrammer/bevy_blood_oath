pub trait ActionMenu<T> {
    fn actions() -> Vec<T>;
    fn label(&self) -> &'static str;
}
