
/*
#![crate_id = "modbus"]
#![crate_type = "lib"]
*/

extern crate modbus_sys;
extern crate libc;
use std::ffi::{CString, CStr};

use std::net::{SocketAddrV4};

use libc::c_uint;

fn octets_to_str(oct : &[u8; 4]) -> String
{
    format!("{}.{}.{}.{}", oct[0], oct[1], oct[2], oct[3])
}

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

pub fn modbus_new_tcp(ip_address: *const libc::c_char, port: libc::c_int) -> *mut modbus_sys::modbus_t
{
    unsafe {
        modbus_sys::modbus_new_tcp(ip_address, port)
    }
}


pub struct Modbus {
    handle: *mut modbus_sys::modbus_t,
}

impl Modbus {

    pub fn new_tcp(addr: &SocketAddrV4) -> Modbus
    {
        let addr_str = octets_to_str( &(addr.ip().octets()) );

        println!(" addr_str = {} ", addr_str);

        unsafe {
            let handle = modbus_sys::modbus_new_tcp(
                CString::new(addr_str).unwrap().as_ptr(),
                addr.port() as i32
            );

            assert!(!handle.is_null());

            let mut ret = Modbus {
                handle: handle,
            };

            return ret

        }

    }

}

