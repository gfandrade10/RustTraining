use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::ptr;


trait Dummy
{
    type Item;
    fn print_object(&self) -> &Self::Item;
}

struct DummyStr<'a>
{
    name: &'a str,
    count: usize,
}

impl<'a> Dummy for DummyStr<'a>
{
    type Item = DummyStr<'a>;
    
    fn print_object(&self) -> &Self::Item
    {
        println!("Name: {}\nAge: {}\n",
            self.name,
            self.count
        );
        self
    }
}

impl<'a> DummyStr<'a>
{
    fn _new() -> DummyStr<'a>
    {
        DummyStr { name: "", count: 0, }
    }

    fn from(i_name: &'a str, i_count: usize) -> DummyStr<'a>
    {
        DummyStr { name: i_name, count: i_count, }
    }
}

fn main() 
{
    let layout = Layout::array::<DummyStr>(1).unwrap();
    let ptr = unsafe { alloc(layout) as *mut c_void };

    unsafe { 
        ptr::write(ptr as *mut DummyStr, DummyStr::from("Raw", 0)); 
        (*(ptr as *mut DummyStr)).print_object();
    }

    let dummy = DummyStr::from("Gui", 36);
    dummy.print_object();

    unsafe {
        ptr::drop_in_place(ptr);
        dealloc(ptr as *mut u8, layout);
    }
}
