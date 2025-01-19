pub trait Typed<T> {
    pub fn ty() -> Result<T, ()>;
}
