extern crate nes;

use nes::mem::*;

#[test]
fn it_tests_ram_store() {
    let mut ram = Ram { val: [0; 0x800] };
    let expected: u8 = 200;
    let address = 0x2000;

    ram.storeb(address, expected);

    assert_eq!(ram.loadb(address), expected);
}

#[test]
fn it_tests_ram_load_null() {
    let mut ram = Ram { val: [0; 0x800] };
    let address = 0x2000;

    assert_eq!(ram.loadb(address), 0);
}
