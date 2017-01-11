
extern crate modbus_sys;
extern crate libc;

use libc::c_uint;

pub fn get_major_version() -> c_uint
{
    modbus_sys::libmodbus_version_major
}
pub fn get_minor_version() -> c_uint
{
    modbus_sys::libmodbus_version_minor
}
pub fn get_patch_version() -> c_uint
{
    modbus_sys::libmodbus_version_micro
}

pub fn modbus_new_tcp(ip_address: *const c_char, port: c_int) -> *mut modbus_t
{
    modbus_sys::modbus_new_tcp(ip_address: *const c_char, port: c_int)
}


