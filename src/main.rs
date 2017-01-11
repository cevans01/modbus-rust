
extern crate modbus;
use modbus::Modbus;

pub fn main() {

    let addr = "127.0.0.1:12345".parse().unwrap();
    let mut m = Modbus::new_tcp(&addr);

}
