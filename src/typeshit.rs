use std::marker::PhantomData;

pub type TypeShitHost<T> = PhantomData<T>;
pub struct TypeShit(pub Vec<Box<TypeShitHost<()>>>);

impl TypeShit {
    pub fn get<T>(&mut self, id: usize) -> Option<Box<T>> {
        unsafe { std::mem::transmute(self.0.get(id)) }
    }

    pub fn push<T>(&mut self, b: Box<T>) -> usize {
        let index = self.0.len();
        self.0.push(unsafe { std::mem::transmute(b) });
        index
    }

    pub fn pop(&mut self, id: usize) {
        if id < self.0.len() {
            self.0.remove(id);
        } else {
            eprintln!("tried to pop off typeshit when couldn't :(");
        }
    }
}