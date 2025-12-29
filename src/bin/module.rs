#![allow(unused)]

//modules are a way to organize code in rust.

// base modules
mod math {
    pub struct Network {
        pub connected: bool,
        pub timeout_err: String,
        pub not_connected: bool,
    }
}

fn main() {
    //base
    let check = math::Network {
        connected: true,
        timeout_err: String::from("Timeout"),
        not_connected: false,
    };
    let hmm = check.connected;
    let ok = check.timeout_err;

    println!("connected: {}", check.not_connected);
    println!("connected: {}", hmm);
    println!("connected: {}", ok);
}
