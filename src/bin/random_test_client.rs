
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
    let mut tab_rp_registers = vec![0u16; nb];

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
            if let Err(e) = mb.write_bit(addr, tab_rq_bits[0] as i32) {
                println!("Error: Modbus::write_bit");
                println!("{}", e);
                nb_fail += 1;
            }
            else {
                if let Err(e) = mb.read_bits(addr, &mut tab_rp_bits[0..1]) {
                    println!("Error: Modbus::read_bits");
                    println!("{}", e);
                    nb_fail += 1;
                }
                else {
                    if tab_rq_bits[0] != tab_rp_bits[0] {
                        println!("Error: write_bit/read_bits mismatch");
                        nb_fail += 1;
                    }
                }
            }

            /* MULTIPLE BITS */
            if let Err(e) = mb.write_bits(addr, &tab_rq_bits[..]) {
                println!("Error: Modbus::write_bits");
                println!("{}", e);
                nb_fail += 1;
            }
            else {
                if let Err(e) = mb.read_bits(addr, &mut tab_rp_bits[..]) {
                    println!("Error: Modbus::read_bits");
                    println!("{}", e);
                    nb_fail += 1;
                }
                else {
                    for (i, item) in tab_rq_bits.iter().enumerate() {
                        if *item != tab_rp_bits[i] {
                            println!("Error: Modbus write_bits/read_bits: for index {}, read {}, write {}",
                                     i, tab_rp_bits[i], item);
                            nb_fail += 1;
                        }
                    }
                }
            }


            /* SINGLE REGISTER */
            if let Err(e) = mb.write_register(addr, tab_rq_registers[0] as i32) {
                println!("Error: Modbus::write_register");
                println!("{}", e);
                nb_fail += 1;
            }
            else {
                if let Err(e) = mb.read_registers(addr, &mut tab_rp_registers[0..1]) {
                    println!("Error: Modbus::read_registers");
                    println!("{}", e);
                    nb_fail += 1;
                }
                else {
                    if tab_rq_registers[0] != tab_rp_registers[0] {
                        println!("Error: write_register/read_registers mismatch");
                        nb_fail += 1;
                    }
                }
            }

            /* MULTIPLE REGISTERS */
            if let Err(e) = mb.write_registers(addr, &tab_rq_registers[..]) {
                println!("Error: Modbus::write_registers");
                println!("{}", e);
                nb_fail += 1;
            }
            else {
                if let Err(e) = mb.read_registers(addr, &mut tab_rp_registers[..]) {
                    println!("Error: Modbus::read_registers");
                    println!("{}", e);
                    nb_fail += 1;
                }
                else {
                    for (i, item) in tab_rq_registers.iter().enumerate() {
                        if *item != tab_rp_registers[i] {
                            println!("Error: Modbus write_registers/read_registers: for index {}, read {}, write {}",
                                     i, tab_rp_registers[i], item);
                            nb_fail += 1;
                        }
                    }
                }
            }

            /* R/W MULTIPLE REGISTERS */
            match mb.write_and_read_registers(addr, &tab_rq_registers[..],
                                                        addr, &mut tab_rp_registers[..]) {
                Ok(rv) =>
                    if rv != nb as i32 {
                        println!("Error: Modbus::write_and_read_registers mismatch");
                        nb_fail += 1;
                    }
                    else {
                        for (i, item) in tab_rq_registers.iter().enumerate() {
                            if *item != tab_rp_registers[i] {
                                println!("Error: Modbus write_registers/read_registers: for index {}, read {}, write {}",
                                         i, tab_rp_registers[i], item);
                                nb_fail += 1;
                            }
                        }
                    },
                Err(e) =>
                    println!("Error: Modbus::write_and_read_registers"),
            }
        }

        println!("Test: ");
        match nb_fail {
            0 => println!("SUCCESS"),
            x => println!("{} FAILS", x),
        }
    }
}
