use std::ops::DerefMut;
use crate::memory::{Heap, Page, Stack};
use crate::util::{Generic};
use crate::object::{Object, prototype, init as init_object};
use std::sync::Arc;

static mut PROTO_POOL: Vec<Arc<Option<&Object>>>= Vec::new();

macro_rules! build_proto {
    ($proto: expr => $cb: expr) => {{
        let mut proto = Object::new();
        proto.set_object(0, $proto);
        proto.set_fn(1, $cb);
        proto.set_fn(2, generic_proto_cb);
        let mut storage = Arc::from(Some(proto).as_deref());
        unsafe {
            PROTO_POOL.push(storage);
            PROTO_POOL.get_unchecked(PROTO_POOL.len() - 1).as_deref()
        }
    }}
}

macro_rules! make_cb {
    ($name:ident ($x: ident) => $body: expr) => {
        pub(crate) fn $name($x: Option<&Object>) -> Generic {
            let obj = { $body };
            Ok(obj)
        }
    }
}

make_cb!(generic_proto_cb (x) => {
    dbg!(format!("[proto-cb] {:?}", x).as_str());
    x
});

fn generic_proto<'a>() -> Option<&'a Object> {
    build_proto!(prototype() => generic_proto_cb)
}

macro_rules! make_init {
    ($static_name: ident, $name:ident: $func:expr => $cb:expr) => {
        static mut $static_name: Arc<Option<&Object>> = Arc::from(None);
        pub(crate) fn $name<'a>() -> Option<&'a Object> {
            { $cb }
            unsafe {
                if $static_name.is_none() {
                    $static_name = build_proto!(unsafe { generic_proto().unwrap_unchecked() } => $func);
                }
                &$static_name
            }
        }
    }
}

make_init!(PROTO_MEMORY, memory: generic_proto_cb => {
    println!("[init] initializing memory subsystem...");
    unsafe {
        crate::memory::ROOT_PAGE = Some(Page::new());
        crate::memory::ROOT_STACK = Some(Stack::new());
        crate::memory::ROOT_HEAP = Some(Heap::new());
        crate::memory::ROOT_OBJECT = Some(Object::new());
    }
});

make_cb!(list_proto_index (x) => {
    let obj = x.expect("failed to get object of type 'list'");
    let getter = unsafe { obj.get_fn(2).unwrap_unchecked() };
    let index = unsafe { obj.get_usize(3).unwrap_unchecked() };
    getter(Some(Object::wrap(index)))
});

make_cb!(list_proto_index_set (x) => {
    let obj = x.expect("failed to get object of type 'list'");
    let setter = unsafe { obj.get_fn(2).unwrap_unchecked() };
    let index = unsafe { obj.get_usize(3).unwrap_unchecked() };
    setter(Some(Object::wrap(index)))
});

make_init!(PROTO_LIST, list: list_proto_index => {
    println!("[init/type] registering list type...");
});

pub fn init() {
    init_object();
    memory();
    list();
}