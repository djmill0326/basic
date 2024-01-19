use crate::{memory::Dynamic, util::{Output, Generic}};

#[derive(Clone, Debug)]
pub struct Object(Dynamic);

static mut CALL_TABLE: Vec<fn (Option<&Object>) -> Generic> = Vec::new();

fn call_table() -> &'static mut Vec<fn (Option<&Object>) -> Generic> {
    unsafe { &mut CALL_TABLE }
}

impl Object {
    pub fn new() -> Object {
        Object(Dynamic::new())
    }

    pub fn get_u32(&self, index: usize) -> Output<u32> {
        if index < 4 {
            Ok(unsafe { *self.0.0.get_unchecked(index) })
        } else {
            self.0.1.get(index - 4).map_or(Err("failed to get object property"), |x| Ok(*x))
        }
    }

    pub fn set_u32(&mut self, index: usize, x: u32) -> Output<()> {
        if index < 4 {
            unsafe { 
                let addr = self.0.0.get_unchecked_mut(index);
                *addr = x;
                Ok(())
            }
        } else {
            if index < self.0.1.len() {
                self.0.1[index - 4] = x;
                Ok(())
            } else if index == self.0.1.len() {
                self.0.1.push(x);
                Ok(())
            } else {
                Err("tried to set out of bounds object property")
            }
        }
    }

    pub fn get_object<'a>(&self, index: usize) -> Output<&'a Object> {
        let x = self.get_u32(index)? as usize;
        unsafe { Ok(std::mem::transmute::<usize, &Object>(x)) }
    }

    pub fn set_object<'a>(&mut self, index: usize, x: &Object) -> Output<()> {
        let inner = unsafe { std::mem::transmute::<&Object, usize>(x) } as u32;
        self.set_u32(index, inner)
    }

    pub fn get_fn(&self, index: usize) -> Output<fn (Option<&Object>) -> Generic> {
        let index = self.get_u32(index).expect("failed to get object property for function call") as usize;
        call_table()
            .get(self.get_u32(index)? as usize)
            .map_or(Err("failed to get object property as function"), |x| Ok(*x))
    }

    pub fn set_fn<'a>(&mut self, index: usize, x: fn (Option<&Object>) -> Generic) -> Output<()> {
        call_table().push(x);
        self.set_u32(index, (call_table().len() - 1) as u32)
    }

    pub fn call<'a>(&self, x: Option<&'a Object>) -> Generic<'a> {
        self.get_fn(0).expect("failed to get object as lambda")(x)
    }
}

macro_rules! closure {
    ($x: expr) => {
        unsafe {{
            let func: &dyn Fn(Option<&crate::object::Object>) -> crate::util::Generic = &$x;
            let x: (fn (Option<&crate::object::Object>) -> crate::util::Generic, u64) = std::mem::transmute(func);
            x.0
        }}
    }
}

macro_rules! out {
    ($x: expr) => {
        Ok::<Option<&crate::object::Object>, &str>($x)
    }
}