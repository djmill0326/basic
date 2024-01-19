use crate::{memory::Dynamic, util::{Output, Generic}};

fn proto_call(x: Option<&Object>) -> Generic {
    println!(";) [proto_call] {:?}", x);
    Ok(x)
}

static mut OBJ_ROOT: Option<Object> = None;

fn _obj_root() -> &'static Object {
    unsafe { OBJ_ROOT.as_ref().unwrap_unchecked() }
}

pub fn prototype() -> Object {
    let mut obj = Object(Dynamic::new());
    obj.set_object(0, _obj_root()).unwrap();
    obj.set_fn(1, proto_call).unwrap();
    obj
}

#[derive(Clone, Debug)]
pub struct Object(pub Dynamic);

static mut CALL_TABLE: Vec<fn (Option<&Object>) -> Generic> = Vec::new();

fn call_table() -> &'static mut Vec<fn (Option<&Object>) -> Generic> {
    unsafe { &mut CALL_TABLE }
}

impl Object {
    pub fn new() -> Object {
        let mut obj = Object(Dynamic::new());
        unsafe {
            obj.set_object(0, &prototype()).unwrap_unchecked();
            obj.set_fn(1, proto_call).unwrap_unchecked();
        }
        obj
    }

    pub fn from(x: &Object) -> Object {
        let mut obj = Object::new();
        obj.set_object(0, x);
        obj
    }

    pub fn get_usize(&self, index: usize) -> Output<usize> {
        if index < 4 {
            Ok(unsafe { *self.0.0.get_unchecked(index) })
        } else {
            self.0.1.get(index - 4).map_or(Err("failed to get object property"), |x| Ok(*x))
        }
    }

    pub fn set_usize(&mut self, index: usize, x: usize) -> Output<()> {
        if index < 4 {
            unsafe { 
                *self.0.0.get_unchecked_mut(index) = x;
                Ok(())
            }
        } else {
            if index - 4 < self.0.1.len() {
                self.0.1[index - 4] = x;
                Ok(())
            } else if index - 4 == self.0.1.len() {
                self.0.1.push(x);
                Ok(())
            } else {
                Err("tried to set out of bounds object property")
            }
        }
    }

    pub fn get_object<'a>(&self, index: usize) -> Output<&'a Object> {
        let x = self.get_usize(index)?;
        unsafe { Ok(std::mem::transmute::<usize, &Object>(x)) }
    }

    pub fn set_object<'a>(&mut self, index: usize, x: &Object) -> Output<()> {
        let inner = unsafe { std::mem::transmute::<&Object, usize>(x) };
        self.set_usize(index, inner)
    }

    pub fn get_fn(&self, index: usize) -> Output<fn (Option<&Object>) -> Generic> {
        let index = self.get_usize(index).expect("failed to get object property for function call");
        call_table()
            .get(self.get_usize(index)?)
            .map_or(Err("failed to get object property as function"), |x| Ok(*x))
    }

    pub fn set_fn<'a>(&mut self, index: usize, x: fn (Option<&Object>) -> Generic) -> Output<()> {
        call_table().push(x);
        self.set_usize(index, (call_table().len() - 1))
    }

    pub fn call<'a>(&self, x: Option<&'a Object>) -> Generic<'a> {
        unsafe { self.get_fn(1).unwrap_unchecked()(x) }
    }

    pub fn prototype(&self) -> &Object {
        unsafe { self.get_object(0).unwrap_unchecked() }
    }

    pub fn str(&self) -> String {
        format!("{:?}", self)
    }
}

pub(crate) fn init() {
    unsafe { OBJ_ROOT = Some(Object(Dynamic::new())) }
}