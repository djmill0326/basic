use std::marker::{PhantomData, PhantomPinned};

use crate::{object::Object, memory::Dynamic, util::Generic};

static mut LIST_PROTO: Option<Object> = None;
static mut LISTS: Vec<Vec<PhantomPinned>> = Vec::new();

fn type_erase<T>(x: Vec<T>) -> Vec<PhantomPinned> {
    unsafe { std::mem::transmute(x) }
}

pub fn list_proto() -> &'static Object {
    unsafe { LIST_PROTO.as_ref().unwrap_unchecked() }
}

pub fn list<'a, T>(x: Vec<T>) -> &'a mut Object {
    unsafe {
        LISTS.push(type_erase(x));
        let mut obj = Object::from(list_proto());
        obj.set_usize(2, LISTS.len() - 1).unwrap_unchecked();
        obj
    }
}

pub fn index(x: Option<&Object>) -> Generic {
    println!("[core-list] {:?}", x);
    Ok(x)
}