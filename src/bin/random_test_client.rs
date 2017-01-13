
extern crate rand;
extern crate errno;
use errno::{Errno, errno};

extern crate modbus;
use modbus::{Modbus, ModbusMapping, ModbusResult};
use rand::{thread_rng, Rng};

/* The goal of this program is to check all major functions of
   libmodbus:
   - write_coil
   - read_bits
   - write_coils
   - write_register
   - read_registers
   - write_registers
   - read_registers

   All these functions are called with random values on a address
   range defined by the following defines.
*/

const LOOP: i32             =   1;
const SERVER_ID: i32        =   17;
const ADDRESS_START: i32    =   0;
const ADDRESS_END: i32      =   99;

/*
macro_rules! log_try {
    ($e:expr,$v:ident) => (match $e {
        Ok(val) => val,
        Err(err) => {
            $v += 1;
            return Err(::std::convert::From::from(err))
        },
    });
}
*/
/*
macro_rules! log_try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
*/

fn log_res( result: ModbusResult, log_var: &mut u32 ) -> ModbusResult
{
    match result {
        Ok(_) => return result,
        Err(_) => { *log_var = *log_var + 1; return result }
    }
}

pub fn main() {

    let addr = "127.0.0.1:1502".parse().unwrap();
    let mut mb = Modbus::new_tcp(&addr);

    mb.set_debug(true);

    mb.connect();


    let nb = (ADDRESS_END - ADDRESS_START) as usize;
    let mut nb_fail: u32 = 0;

    let mut tab_rp_bits = vec![0u8; nb];

    for nb_loop in 0..LOOP {
        for addr in ADDRESS_START..ADDRESS_END {

            let tab_rq_registers = rand::thread_rng()
                                    .gen_iter::<u16>()
                                    .take(nb).collect::<Vec<u16>>();

            let tab_rw_rq_registers = tab_rq_registers.iter()
                                    .map(|&x| !x).collect::<Vec<u16>>();

            let tab_rq_bits = tab_rw_rq_registers.iter()
                                    .map(|&x| (x % 2) as u8).collect::<Vec<u8>>();


            /* WRITE BIT */
            /*
            match mb.write_bit(addr, tab_rq_bits[0]) {
                Ok(()) => _,
                Err(err) =>
            }
            */
            /*
            if mb.write_bit(addr, tab_rq_bits[0] as i32).is_ok() {
            }
            else {

            }
            */
            //log_try!( mb.write_bit(addr, tab_rq_bits[0] as i32); nb_fail );
            //log_try!( mb.write_bit(addr, tab_rq_bits[0] as i32) );
            //try!( mb.write_bit(addr, tab_rq_bits[0] as i32) );
            //
            if let Err(e) = mb.write_bit(addr, tab_rq_bits[0] as i32) {
                println!("Error happened:");
                println!("{}", e);
            }
            /*
            if log_res( mb.write_bit(addr, tab_rq_bits[0] as i32), &mut nb_fail ).is_ok() {
                log_res( mb.read_bits(addr, &mut tab_rp_bits[0..1]), &mut nb_fail );
            }
            */

            /*
            if log_res( mb.write_bits(addr, &tab_rq_bits[..]), &mut nb_fail ).is_ok() {
                log_res( mb.read_bits(addr, &mut tab_rp_bits[..]), &mut nb_fail );
            }
            */

        }

        println!( "nb_fail is = {}", nb_fail );


    }
}
