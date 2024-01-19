use crate::memory::{object::Object, Dynamic, list::{list_proto, self}};
use crate::util::Generic;

static mut STR_PTOTO: Option<Object> = None;

fn str_proto() -> &'static Object {
    unsafe { STR_PTOTO.as_ref().unwrap_unchecked() }
}

pub fn str<'a>(x: impl From<&'a str>) -> Object {
    let obj = Object::from(str_proto());
    obj
}

fn proto_call(x: Option<&Object>) -> Generic {
    println!("[core-string] {:?}", x);
    Ok(x)
}

pub(crate) fn init() {
    list::init();
    unsafe {
        let mut obj = Object(Dynamic::new());
        obj.set_fn(1, proto_call).unwrap();
        obj.set_object(2, list_proto()).unwrap();
        STR_PTOTO = Some(obj);
    }
}