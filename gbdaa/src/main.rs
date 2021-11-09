extern crate timer;
extern crate chrono;

use std::io;
use std::net::UdpSocket;
use timer::Timer;
use chrono::Duration;
use std::thread;

const DAA_RATE_MS: i64 = 5000;

fn rcv() {
    println!("Receive thread started");
    let socket = UdpSocket::bind("0.0.0.0:4242").expect("Cannot bind socket");
    let mut buf = [0; 128];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Cannot receive data");
        println!("Received {} from {}", amt, src)
    }
}

fn daa() {
    println!("DAA Cycle")
}

fn main() {
    let rcv_handle = thread::spawn(rcv);
    let timer = Timer::new();
    let repeat = Duration::milliseconds(DAA_RATE_MS);
    let guard = timer.schedule_repeating(repeat, daa);

    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to read command");

        if cmd.contains("exit") {
            break;
        }
    }

    drop(guard);
    rcv_handle.join().unwrap();
}
