
/*
#![crate_id = "modbus"]
#![crate_type = "lib"]
*/

extern crate modbus_sys;
extern crate libc;
extern crate errno;

use std::ffi::{CString, CStr};

use std::net::{SocketAddrV4};

use libc::{c_uint, c_int};
use errno::{Errno, errno};

// Helpers
pub type ModbusResult = Result<i32, Errno>;

fn octets_to_str(oct : &[u8; 4]) -> String
{
    format!("{}.{}.{}.{}", oct[0], oct[1], oct[2], oct[3])
}

fn cvt(r: c_int) -> ModbusResult {
    if r == -1 {
        Err(errno())
    } else {
        Ok(r as i32)
    }
}

// Free functions
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

/*
pub fn modbus_new_tcp(ip_address: *const libc::c_char, port: libc::c_int) -> *mut modbus_sys::modbus_t
{
    unsafe {
        modbus_sys::modbus_new_tcp(ip_address, port)
    }
}
*/


// STRUCTS
//
// ModbusMaping
pub struct ModbusMapping {
    handle: *mut modbus_sys::modbus_mapping_t,
}

impl ModbusMapping {
    pub fn new(nb_bits: c_int,
                  nb_input_bits: c_int,
                  nb_registers: c_int,
                  nb_input_registers: c_int) -> ModbusMapping
    {
        unsafe {
            let handle = modbus_sys::modbus_mapping_new(
                            nb_bits, nb_input_bits,
                            nb_registers, nb_input_registers);
            let mapping = ModbusMapping{ handle: handle };
            return mapping
        }
    }

    pub fn new_start_address( start_bits: c_uint,
                                nb_bits: c_uint,
                                start_input_bits: c_uint,
                                nb_input_bits: c_uint,
                                start_registers: c_uint,
                                nb_registers: c_uint,
                                start_input_registers: c_uint,
                                nb_input_registers: c_uint) -> ModbusMapping
    {
        unsafe {
            let handle = modbus_sys::modbus_mapping_new_start_address(
                                start_bits, nb_bits, start_input_bits,
                                nb_input_bits, start_registers, nb_registers,
                                start_input_registers, nb_input_registers);
            let mapping = ModbusMapping{ handle: handle };
            return mapping

        }
    }
}

impl Drop for ModbusMapping {
    fn drop(&mut self) {
        unsafe {
            modbus_sys::modbus_mapping_free(self.handle);
        }
    }
}


// Modbus
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

    pub fn set_debug(&mut self, flag: bool)
    {
        let flag_i: c_int = match flag { true => 1, false => 0 };
        unsafe {
            modbus_sys::modbus_set_debug(self.handle, flag_i);
        }
    }

    pub fn connect(&mut self) -> ModbusResult
    {
        unsafe {
            let r = modbus_sys::modbus_connect(self.handle);
            return cvt(r)
        }
    }

    pub fn close(&mut self)
    {
        unsafe {
            modbus_sys::modbus_close(self.handle)
        }
    }

    // Write / read bits
    pub fn write_bit(&mut self, coil_addr: c_int, status: c_int) -> ModbusResult
    {
        unsafe {
            let r = modbus_sys::modbus_write_bit(self.handle, coil_addr, status);
            return cvt(r)
        }
    }
    pub fn write_bits(&mut self, addr: c_int, data: &[u8]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_bits(self.handle, addr, data.len() as c_int, data.as_ptr()) )
        }

    }

    pub fn read_bits(&mut self, addr: c_int, dest: &mut [u8]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_bits(self.handle,
                                                 addr, dest.len() as c_int,
                                                 dest.as_mut_ptr())
                 )
        }
    }

    // Write / read registers
    pub fn write_register(&mut self, reg_addr: c_int, value: c_int) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_register(self.handle, reg_addr, value) )
        }
    }

    pub fn write_registers(&mut self, addr: c_int, data: &[u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_registers(self.handle, addr, data.len() as i32, data.as_ptr()) )
        }
    }

    pub fn read_registers(&mut self, addr: c_int, dest: &mut [u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_registers(self.handle, addr, dest.len() as i32, dest.as_mut_ptr()) )
        }

    }

    pub fn read_input_registers(&mut self, addr: c_int, dest: &mut [u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_input_registers(self.handle, addr, dest.len() as i32, dest.as_mut_ptr()) )
        }
    }

    pub fn write_and_read_registers(&mut self, write_addr: c_int, src: &[u16],
                                           read_addr: c_int, dest: &mut [u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_and_read_registers(self.handle, write_addr, src.len() as i32, src.as_ptr(),
                                                                    read_addr, dest.len() as i32, dest.as_mut_ptr()) )
        }
    }

}

impl Drop for Modbus {
    fn drop(&mut self) {
        unsafe {
            modbus_sys::modbus_free(self.handle);
        }
    }
}

