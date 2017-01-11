
#[cfg(test)]
pub mod tests {
    extern crate modbus;

    #[test]
    fn test_scratchpad() {
        //let m = modbus::Modbus{};
        //m.new_tcp();
        assert!(modbus::get_major_version() == 3);
    }



}

