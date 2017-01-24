
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

/// Returns the Major version number of the libmodbus library
pub fn get_major_version() -> c_uint
{
    modbus_sys::libmodbus_version_major
}
/// Returns the Minor version number of the libmodbus library
pub fn get_minor_version() -> c_uint
{
    modbus_sys::libmodbus_version_minor
}
/// Returns the Patch version number of the libmodbus library
pub fn get_patch_version() -> c_uint
{
    modbus_sys::libmodbus_version_micro
}

/// Mapping struct which servers can use to reply to clients
pub struct ModbusMapping {
    handle: *mut modbus_sys::modbus_mapping_t,
}

impl ModbusMapping {
    /// Create a new mapping. This will allocate four arrays to store bits, input bits, registers
    /// and input registers. All values are initialized to zero.
    /// # Example
    /// ```
    /// use modbus::ModbusMapping;
    ///
    /// let mbm = ModbusMapping::new(500, 500, 500, 500);
    /// ```
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

    /// Create a new mapping. This will allocate four arrays to store bits, input bits, registers
    /// and input registers. All values are initialized to zero.
    /// # Example
    /// ```
    /// use modbus::ModbusMapping;
    ///
    /// let mbm = ModbusMapping::new_start_address(0, 500, 0, 500, 0, 500, 0, 500);
    /// ```
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

/// Frees a ModbusMapping
impl Drop for ModbusMapping {
    fn drop(&mut self) {
        unsafe {
            modbus_sys::modbus_mapping_free(self.handle);
        }
    }
}

unsafe impl Send for ModbusMapping { }
unsafe impl Sync for ModbusMapping { }


/// Context for modbus functions
pub struct Modbus {
    handle: *mut modbus_sys::modbus_t,
}

impl Modbus {

    /// Create a new Modbus context for TCP/IPv4
    ///
    /// # Arguments
    /// * `addr` - A TCP/IPv4 socket address
    ///
    /// # Example
    ///
    /// ```
    /// use modbus::Modbus;
    /// let addr = "127.0.0.1:1502".parse().unwrap();
    /// let mut mb = Modbus::new_tcp(&addr);
    /// ```
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

    /// Set debug flag of the context
    pub fn set_debug(&mut self, flag: bool)
    {
        let flag_i: c_int = match flag { true => 1, false => 0 };
        unsafe {
            modbus_sys::modbus_set_debug(self.handle, flag_i);
        }
    }

    /// Establish a connection to a Modbus server
    pub fn connect(&mut self) -> ModbusResult
    {
        unsafe {
            let r = modbus_sys::modbus_connect(self.handle);
            return cvt(r)
        }
    }

    /// Close a modbus connection
    ///
    /// This should be called if you have previously called Modbus::connect
    pub fn close(&mut self)
    {
        unsafe {
            modbus_sys::modbus_close(self.handle)
        }
    }

    /// Write a single bit
    ///
    /// This function will write the status of status at the address addr of the remote device. The
    /// value must me set to 1 or 0.
    ///
    /// The function uses the Modbus function code 0x05 (force single coil).
    pub fn write_bit(&mut self, coil_addr: c_int, status: c_int) -> ModbusResult
    {
        unsafe {
            let r = modbus_sys::modbus_write_bit(self.handle, coil_addr, status);
            return cvt(r)
        }
    }

    /// Write many bits
    ///
    /// This function shall write the status of the data.len() bits from data at the address addr
    /// of the remote device. The data slice must contain bytes set to 1 or 0.
    ///
    /// The function uses the Modbus function code 0x0F (force multiple coils).
    pub fn write_bits(&mut self, addr: c_int, data: &[u8]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_bits(self.handle, addr, data.len() as c_int, data.as_ptr()) )
        }

    }

    /// Read many bits
    ///
    /// This function shall read the status of the dest.len() bits (coils) to the address
    /// addr of the remote device. The result of reading is stored in dest slice as u8
    /// set to 1 or 0.
    ///
    /// The function uses the Modbus function code 0x01 (read coil status).
    ///
    pub fn read_bits(&mut self, addr: c_int, dest: &mut [u8]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_bits(self.handle,
                                                 addr, dest.len() as c_int,
                                                 dest.as_mut_ptr())
                 )
        }
    }

    /// Write a single register
    ///
    /// This function shall write the value of value holding registers at
    /// the address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x06 (preset single register).
    ///
    pub fn write_register(&mut self, reg_addr: c_int, value: c_int) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_register(self.handle, reg_addr, value) )
        }
    }

    /// Write many registers
    ///
    /// This function shall write the content of the data.len() holding registers
    /// from the array data at address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x10 (preset multiple registers).
    ///
    pub fn write_registers(&mut self, addr: c_int, data: &[u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_write_registers(self.handle, addr, data.len() as i32, data.as_ptr()) )
        }
    }

    /// Read many registers
    ///
    /// This function shall read the content of the dest.len() holding registers to
    /// the address addr of the remote device. The result of reading is stored in dest array as
    /// word values (u16).
    ///
    /// The function uses the Modbus function code 0x03 (read holding registers).
    ///
    pub fn read_registers(&mut self, addr: c_int, dest: &mut [u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_registers(self.handle, addr, dest.len() as i32, dest.as_mut_ptr()) )
        }

    }

    /// Read many input registers
    ///
    /// This function shall read the content of the dest.len() input registers
    /// to address addr of the remote device. The result of the reading is stored in dest array as
    /// word values (u16).
    ///
    /// The function uses the Modbus function code 0x04 (read input registers). The holding
    /// registers and input registers have different historical meaning, but nowadays it’s more
    /// common to use holding registers only.
    ///
    pub fn read_input_registers(&mut self, addr: c_int, dest: &mut [u16]) -> ModbusResult
    {
        unsafe {
            cvt( modbus_sys::modbus_read_input_registers(self.handle, addr, dest.len() as i32, dest.as_mut_ptr()) )
        }
    }

    /// Write and read many registers in a single transaction
    ///
    /// This function shall write the content of the src.len()
    /// holding registers from the slice src to the address write_addr of the remote device then
    /// shall read the content of the dest.len() holding registers to the address read_addr of the
    /// remote device. The result of reading is stored in dest array as word values (u16).
    ///
    /// The function uses the Modbus function code 0x17 (write/read registers).
    ///
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

unsafe impl Send for Modbus { }
unsafe impl Sync for Modbus { }

