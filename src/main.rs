use brainfuck_interpreter::BrainfuckInstructions::{
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

fn main() {
    let instructions = vec![
        IncreasePointer,
        DecreasePointer,
        IncreaseValue,
        DecreaseValue,
        BeginLoop,
        EndLoop,
        PrintCell,
        WriteToCell,
    ];

    let _program = BrainfuckProgram::new(instructions);
}
