use crate::{memory::Dynamic, util::{Output, Generic}};

fn proto_call(x: Option<&Object>) -> Generic {
    println!(";) [proto_call] {:?}", x);
    Ok(x)
}

static mut OBJ_ROOT: Option<Object> = None;
static mut PROTO_ROOT: Option<Object> = None;

fn _obj_root() -> &'static Object {
    unsafe { OBJ_ROOT.as_ref().unwrap_unchecked() }
}

pub fn init_prototype() {
    let mut obj = Object(Dynamic::new());
    obj.set_object_unchecked(0, _obj_root());
    obj.set_fn_unchecked(1, proto_call);
    unsafe { PROTO_ROOT = Some(obj); }
}

pub(crate) fn init() {
    unsafe {
        OBJ_ROOT = Some(Object(Dynamic::new()));
        init_prototype();
    }
}

pub fn prototype() -> &'static Object {
    unsafe { PROTO_ROOT.as_ref().unwrap_unchecked() }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object(pub Dynamic);

static mut OBJECT_TABLE: Vec<Object> = Vec::new();
static mut CALL_TABLE: Vec<fn (Option<&Object>) -> Generic> = Vec::new();

fn object_table() -> &'static mut Vec<Object> { unsafe { &mut OBJECT_TABLE } }
fn call_table() -> &'static mut Vec<fn (Option<&Object>) -> Generic> {
    unsafe { &mut CALL_TABLE }
}

impl Object {
    pub fn new<'a>() -> &'a mut Object {
        let mut obj = Object(Dynamic::new());
        unsafe {
            obj.set_object_unchecked(0, &prototype());
            obj.set_fn(1, proto_call).unwrap_unchecked();
        object_table().push(obj);
        unsafe { object_table().get_mut(object_table().len() - 1).unwrap_unchecked() }
            }
    }

    pub fn from<'a>(x: &Object) -> &'a mut Object {
        let mut obj = Object::new();
        unsafe { obj.set_object(0, x).unwrap_unchecked() };
        obj
    }

    pub fn wrap<'a>(x: usize) -> &'a mut Object {
        let mut obj = Object::new();
        obj.set_usize_unchecked(2, x);
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

    pub fn set_usize_unchecked(&mut self, index: usize, x: usize) {
        if index < 4 {
            unsafe {
                *self.0.0.get_unchecked_mut(index) = x;
            }
        } else {
            if index - 4 < self.0.1.len() {
                self.0.1[index - 4] = x;
            } else if index - 4 == self.0.1.len() {
                self.0.1.push(x);
            }
        }
    }

    pub fn get_object<'a>(&self, index: usize) -> Output<&'a Object> {
        let x = self.get_usize(index)?;
        unsafe { Ok(std::mem::transmute::<usize, &Object>(x)) }
    }

    pub fn get_object_unchecked<'a>(&self, index: usize) -> &'a Object {
        let x = unsafe {self.get_usize(index).unwrap_unchecked() };
        unsafe { std::mem::transmute::<usize, &Object>(x) }
    }

    pub fn set_object<'a>(&mut self, index: usize, x: &Object) -> Output<()> {
        let inner = unsafe { std::mem::transmute::<&Object, usize>(x) };
        self.set_usize(index, inner)
    }

    pub fn set_object_unchecked(&mut self, index: usize, x: &Object) {
        let inner = unsafe { std::mem::transmute::<&Object, usize>(x) };
        self.set_usize_unchecked(index, inner);
    }

    pub fn get_fn(&self, index: usize) -> Output<fn (Option<&Object>) -> Generic> {
        let index = self.get_usize(index).expect("failed to get object property for function call");
        call_table()
            .get(self.get_usize(index)?)
            .map_or(Err("failed to get object property as function"), |x| Ok(*x))
    }

    pub fn get_fn_unchecked(&self, index: usize) -> Output<fn (Option<&Object>) -> Generic> {
        let index = unsafe { self.get_usize(index).unwrap_unchecked() };
        call_table()
            .get(unsafe { self.get_usize(index).unwrap_unchecked() })
            .map_or(Err("failed to get object property as function"), |x| Ok(*x))
    }

    pub fn set_fn<'a>(&mut self, index: usize, x: fn (Option<&Object>) -> Generic) -> Output<()> {
        call_table().push(x);
        self.set_usize(index, call_table().len() - 1)
    }

    pub fn set_fn_unchecked<'a>(&mut self, index: usize, x: fn (Option<&Object>) -> Generic) {
        call_table().push(x);
        self.set_usize_unchecked(index, call_table().len() - 1);
    }

    pub fn call<'a>(&self, x: Option<&'a Object>) -> Generic<'a> {
        unsafe { self.get_fn(1).unwrap_unchecked()(x) }
    }

    pub fn prototype(&self) -> &Object {
        self.get_object_unchecked(0)
    }

    pub fn str(&self) -> String {
        format!("{:?}", self)
    }
}