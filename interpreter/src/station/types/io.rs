use std::io::{stdin, stdout, Write};

use super::*;

pub static PRINT: StationType = StationType {
    id: "print",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: print_procedure,
};
fn print_procedure(pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    debug_assert!(pallets.len() >= 1, "Invalid argument count");
    match &pallets[0] {
        Pallet::Empty => {}
        Pallet::Bool(b) => {
            if *b {
                print!("true");
            } else {
                print!("false");
            }
        }
        Pallet::Char(c) => {
            print!("{c}");
        }
        Pallet::String(s) => {
            print!("{s}");
        }
        Pallet::Int(i) => {
            print!("{i}");
        }
        Pallet::Float(f) => {
            print!("{f}");
        }
    }
    return Ok(None);
}

pub static PRINTLN: StationType = StationType {
    id: "println",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: println_procedure,
};
fn println_procedure(pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    print_procedure(pallets)?;
    println!();
    return Ok(None);
}

pub static READLN: StationType = StationType {
    id: "readln",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: readln_procedure,
};
fn readln_procedure(_: Vec<Pallet>) -> Result<Option<Pallet>, String> {
    let mut input = String::new();
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {
            return Ok(Some(Pallet::String(if input.ends_with('\n') {
                input.strip_suffix('\n').unwrap().to_owned()
            } else {
                input
            })));
        }
    }
}
