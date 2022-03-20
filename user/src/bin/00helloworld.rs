#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32{
    println!("hello world !(from user_mode)");
    0
}