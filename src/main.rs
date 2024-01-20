mod init;
pub mod memory;
pub mod util;
pub mod object;
pub mod terminal;
pub mod file_io;
pub mod r#type;

use memory::{Memory,Heap};
use object::{Object};
use util::Generic;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
struct Root(isize);

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

#[derive(Copy, Clone, Debug)]
struct File<'a> {
    name: &'a str,
    ptr: Root
}

#[derive(Copy, Clone, Debug)]
struct Io<'a, 'b> {
    send: File<'a>,
    recv: File<'b>
}

#[derive(Copy, Clone, Debug)]
struct Device<'a> {
    io: Io<'a, 'a>
}

#[derive(Copy, Clone, Debug)]
struct Logger<'a> {
    stdout: File<'a>,
    stdin: File<'a>,
    stderr: File<'a>
}

#[derive(Clone, Debug)]
struct Storage {
    device: Device<'static>,
    data: Memory
}

#[derive(Clone, Debug)]
struct System {
    root_device: Device<'static>,
    storage: Storage,
    console: Logger<'static>
}

trait Bootstrap: Sized {
    fn bootstrap() -> Self;
}

impl<'a> File<'a> {
    fn res(name: &'a str, data: isize) -> Self {
        File { name: name, ptr: Root(data) }
    }

    fn stdout() -> Self {
        File::res("stdout", 0)
    }

    fn stdin() -> Self {
        File::res("stdin", 0)
    }

    fn stderr() -> Self {
        File::res("stderr", 0)
    }
}

impl<'a, 'b> Bootstrap for Io<'a, 'b> {
    fn bootstrap() -> Self {
        Self {
            send: File::stdout(),
            recv: File::stdin()
        }
    }
}

impl<'a> Bootstrap for Device<'a> {
    fn bootstrap() -> Self {
        Self { io: Io::bootstrap() }
    }
}

impl<'a> Device<'a> {
    fn storage() -> Storage {
        Storage {
            device: Device::bootstrap(),
            data: Memory::Heap(Heap::new())
        }
    }
}

impl<'a> Bootstrap for Logger<'a> {
    fn bootstrap() -> Self {
        Self {
            stdout: File::stdout(),
            stdin: File::stdin(),
            stderr: File::stderr()
        }
    }
}

impl<'a> Logger<'a> {
    fn log(&self, x: &str) {
        println!("[{}:{}/info] {}", self.stdout.name, self.stdout.ptr, x)
    }

    fn warn(&self, x: &str) {
        println!("[{}:{}/warn] {}", self.stdout.name, self.stdout.ptr, x)
    }

    fn err(&self, x: &str) {
        eprintln!("[{}:{}/error] {}", self.stderr.name, self.stderr.ptr, x)
    }
}

impl Bootstrap for Storage {
    fn bootstrap() -> Self {
        Device::storage()
    }
}

impl Bootstrap for System {
    fn bootstrap() -> Self {
        Self {
            root_device: Device::bootstrap(),
            storage: Storage::bootstrap(),
            console: Logger::bootstrap()
        }
    }
}

impl System {
    fn new() -> Self {
        Self::bootstrap()
    }
}

fn main() {
    init::init();

    let sys = System::new();
    sys.console.log("Hello, world!");

    let mut obj = object::Object::new();
    let mut obj2 = Box::new(Object::new());

    let mut table = r#type::table::Table::new();
    table.set("hello", &mut obj);
    table.set("world", obj2.as_mut());
    
    sys.console.log(&format!("{:?}", table))
}
