pub type Output<T> = Result<T, &'static str>;
pub type Generic<'a> = Output<Option<&'a crate::object::Object>>;