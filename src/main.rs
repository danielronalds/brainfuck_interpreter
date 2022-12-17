use brainfuck_interpreter::BrainfuckInstructions::{
    self,
    BeginLoop,       // [
    DecreasePointer, // <
    DecreaseValue,   // -
    EndLoop,         // ]
    IncreasePointer, // >
    IncreaseValue,   // +
    PrintCell,       // .
    WriteToCell,     // ,
};
use brainfuck_interpreter::BrainfuckProgram;

use clap::Parser;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
/// A brainfuck interpreter written in rust!
#[clap(author, version, about)]
pub struct Args {
    /// The brainfuck program file
    filename: String, 
}

fn main() {
    let args = Args::parse();

    let instructions = file_to_instructions(args.filename);

    let mut program = BrainfuckProgram::new(instructions);

    program.run();
}

fn file_to_instructions(filename: String) -> Vec<BrainfuckInstructions> {
    let mut instructions_vec = Vec::new();

    let file = File::open(filename).expect("Couldn't open file");

    for line in BufReader::new(file).lines() {
        for char in line.expect("Couldn't read line").chars() {
            match char {
                '>' => instructions_vec.push(IncreasePointer),
                '<' => instructions_vec.push(DecreasePointer),
                '+' => instructions_vec.push(IncreaseValue),
                '-' => instructions_vec.push(DecreaseValue),
                '[' => instructions_vec.push(BeginLoop),
                ']' => instructions_vec.push(EndLoop),
                '.' => instructions_vec.push(PrintCell),
                ',' => instructions_vec.push(WriteToCell),
                // Ignore all other chars
                _ => continue
            }
        }
    }

    instructions_vec
}
