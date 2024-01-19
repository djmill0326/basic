
pub mod list;
pub mod object;
pub mod string;
pub mod table;

use object::Object;

const PAGE_SIZE: usize = 2048;
const STACK_SIZE: usize = PAGE_SIZE * 8;

#[derive(Clone, Copy, Debug)]
pub struct Page(pub [usize; PAGE_SIZE]);

#[derive(Clone, Copy, Debug)]
pub struct Stack(pub [usize; STACK_SIZE]);

#[derive(Clone, Debug)]
pub struct Heap(pub Vec<Page>);

#[derive(Clone, Debug)]
pub struct Dynamic(pub [usize;4], pub Vec<usize>);

#[derive(Clone, Debug)]
pub enum Memory {
    Page(Page), Stack(Stack), Heap(Heap), Dynamic(Dynamic)
}

impl Page {
    pub fn new() -> Self { Page([0; PAGE_SIZE]) }
}

impl Stack {
    pub fn new() -> Self { Stack([0; STACK_SIZE]) }
}

impl Heap {
    pub fn new() -> Self { Heap(Vec::new()) }
}

impl Dynamic {
    pub fn new() -> Self { Dynamic([0;4], Vec::new()) }
}

static mut ROOT_PAGE: Option<Page> = None;
static mut ROOT_STACK: Option<Stack> = None;
static mut ROOT_HEAP: Option<Heap> = None;
static mut ROOT_OBJECT: Option<Object> = None;

pub fn init() -> Result<(), &'static str> {
    unsafe {
        ROOT_PAGE = Some(Page::new());
        ROOT_STACK = Some(Stack::new());
        ROOT_HEAP = Some(Heap::new());
        ROOT_OBJECT = Some(Object::new());
    }
    Ok(())
}

pub fn root_page() -> &'static mut Page {
    unsafe { ROOT_PAGE.as_mut().unwrap_unchecked() }
}

pub fn root_stack() -> &'static mut Stack {
    unsafe { ROOT_STACK.as_mut().unwrap_unchecked() }
}

pub fn root_heap() -> &'static mut Heap {
    unsafe { ROOT_HEAP.as_mut().unwrap_unchecked() }
}

pub fn get_page<'a>(x: usize) -> Option<&'a mut Page> {
    root_heap().0.get_mut(x)
}

pub fn alloc_page<'a>() -> (Option<&'a mut Page>, usize) {
    let heap = &mut root_heap().0;
    let index = heap.len();
    heap.push(Page::new());
    (heap.get_mut(index), index)
}

pub fn dealloc_page<'a>(index: usize) -> Result<(), &'static str> {
    root_heap().0.remove(index);
    Ok(())
}