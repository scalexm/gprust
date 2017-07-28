extern crate gprust;

use gprust::{Context, Device, CommandQueue, command_queue};

fn main() {
    let device = Device::default().unwrap();
    let context = Context::default().unwrap();
    let queue = CommandQueue::create(&context, &device, command_queue::Properties::new()).unwrap();
}
