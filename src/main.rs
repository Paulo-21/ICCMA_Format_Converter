use std::{env, process::exit};
mod converter;
use converter::*;

fn print_help() {
    println!("Made by Paul Cibier");
    println!();
    println!("Usage : ./command mode filePath");
    println!();
    println!("Mode 1 : APX -> ICCMA 2023");
    println!("Mode 2 : ICCMA 2023 -> APX");
}
fn main() {
    println!("ICCMA Format Converter");
    let arg = env::args().collect::<Vec<String>>();
    if arg.len() <= 2 {
        print_help();
    }
    else {
        let mode = arg[1].parse().unwrap();
        let filepath = arg[2].clone();
        match mode {
            1 => {
                convert_APX_to_ICCMA2023(filepath);
            },
            2 => {
                convert_ICCMA2023_to_APX(filepath);
            },
            _=> {
                println!("This mode is not suported ");
                print_help();
            }
        }
    }

}
