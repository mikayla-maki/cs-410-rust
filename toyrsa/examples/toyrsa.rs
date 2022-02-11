use std::env;
use toyrsa::*;
///Implements a simple command line interface for the toy RSA lib
fn main() {
    let args: Vec<u64> = env::args()
        .skip(1)
        .map(|arg| arg.parse().unwrap_or_else(|_| error()))
        .collect();

    match args.len() {
        0 => {
            let key = genkey();
            println!(
                "toyrsa: Private key: {:?} \n public key: {}",
                key,
                key.0 as u64 * key.1 as u64
            );
        }
        2 => {
            let data = u32::try_from(args[1]).unwrap_or_else(|_| error());
            let pubkey = args[0];
            println!("toyrsa: Encrypted data: {}", encrypt(pubkey, data));
        }
        3 => {
            let p = u32::try_from(args[0]).unwrap_or_else(|_| error());
            let q = u32::try_from(args[1]).unwrap_or_else(|_| error());
            let data = args[2];
            println!("toyrsa: Decrypted data: {}", decrypt((p, q), data));
        }
        _ => {
            error();
        }
    }
}

///Print out an error message and exit.
fn error() -> ! {
    eprintln!("toyrsa: usage: ");
    eprintln!("toyrsa:        toyrsa");
    eprintln!("toyrsa:        - Generate a new private & public key");
    eprintln!("toyrsa:        toyrsa [pub_key] [data]");
    eprintln!("toyrsa:        - Encrypts a u32's worth of data with the given public key.");
    eprintln!("toyrsa:        toyrsa decrypt [priv_key_p] [priv_key_q] [encrypted data]");
    eprintln!("toyrsa:        - Decrypts a u32's worth of data with the given private key.");

    std::process::exit(1);
}
