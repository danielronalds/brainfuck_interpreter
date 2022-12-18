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

use utf8_read::Reader;

use std::fs::File;

#[derive(Parser, Debug)]
/// A brainfuck interpreter written in rust!
#[clap(author, version, about)]
pub struct Args {
    /// The brainfuck program file
    filename: String,
    /// Whether to debug or not
    #[arg(long)]
    debug: bool
}

fn main() {
    let args = Args::parse();

    let instructions = file_to_instructions(args.filename);

    let mut program = BrainfuckProgram::new(instructions);

    program.run(args.debug);
}

fn file_to_instructions(filename: String) -> Vec<BrainfuckInstructions> {
    let mut instructions_vec = Vec::new();

    let file = File::open(filename).expect("Couldn't open file");

    let mut reader = Reader::new(&file);

    for char in reader.into_iter() {
        let char = char.expect("Couldn't read char?");
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
            _ => continue,
        }
    }

    instructions_vec
}
