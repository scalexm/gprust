extern crate gprust;

use gprust::{Platform, device};

fn main() {
    for p in Platform::list() {
        println!("{:?}", p);
        for d in p.get_devices(device::ALL) {
            println!("{:?}", d);
        }
    }
}
