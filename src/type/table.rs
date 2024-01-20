use crate::object::Object;
use std::collections::HashMap;

type TableTable<'a> = HashMap<&'a str, &'a mut Object>;

#[derive(Debug)]
pub struct Table<'a>(TableTable<'a>);

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: impl Into<&'a str>) -> Option<&&'a mut Object> {
        self.0.get(key.try_into().expect("failed to convert into &str"))
    }

    pub fn set(&mut self, key: impl Into<&'a str>, value: &'a mut Object) -> Option<&'a mut Object> {
        self.0.insert(key.try_into().expect("failed to convert into &str"), value)
    }

    pub fn remove(&mut self, key: impl Into<&'a str>) -> Option<&'a mut Object> {
        self.0.remove(key.try_into().expect("failed to convert into &str"))
    }
}