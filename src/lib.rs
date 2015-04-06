
extern crate libc;

use libc::c_void;
use libc::c_int;
use std::mem;

#[no_mangle]
pub extern fn hello() {
	println!("hello from rust!");
}

//extern {
#[no_mangle]
	pub static hellostr: *const str = "hello stringy\0";
//}

#[no_mangle]
pub extern fn vplay(vpp:*mut *mut c_void) {
	let s: *const str = "void string";
	unsafe { *vpp = s as *mut c_void; }
}

#[no_mangle]
pub extern fn show_string_info() {
	let raw_s: *const str = "raw string";

	println!("string info");
	println!("function ref size {} ", mem::size_of_val(&hello));
	unsafe {
		println!("static string is {}", &*raw_s);
	}
	println!("string literal ref size {} ", mem::size_of_val(&"oh"));

	println!("raw str pointer size {}", mem::size_of_val(&raw_s));
}

//#[no_mangle]
//  pub extern fn rustlib_increment(x: i32) -> i32 { x + 1 }
