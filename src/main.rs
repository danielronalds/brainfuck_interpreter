use brainfuck_interpreter::BrainfuckProgram;
    use brainfuck_interpreter::BrainfuckInstructions::{
        IncreasePointer, // >
        DecreasePointer, // <
        IncreaseValue,   // +
        DecreaseValue,   // -
        BeginLoop,       // [
        EndLoop,         // ]
        PrintCell,       // .
        WriteToCell,     // ,
    };

fn main() {
    let instructions = vec![
        IncreasePointer,
        DecreasePointer,
        IncreaseValue,
        DecreaseValue,
        BeginLoop,
        EndLoop,
        PrintCell,
        WriteToCell
    ];

    let _program = BrainfuckProgram::new(instructions);
}

