pub type Output<T> = Result<T, &'static str>;
pub type OutputRaw = Output<u32>;
pub type Generic<'a> = Output<Option<&'a crate::object::Object>>;