
extern crate modbus;

pub fn main() {
    println!("major = {}", modbus::get_major_version());
    println!("minor = {}", modbus::get_minor_version());
    println!("patch = {}", modbus::get_patch_version());
}
