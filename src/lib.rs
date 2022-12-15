/// Enum for representing all possible brainfuck instructions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrainfuckInstructions {
    IncreasePointer, // >
    DecreasePointer, // <
    IncreaseValue,   // +
    DecreaseValue,   // -
    BeginLoop,       // [
    EndLoop,         // ]
    WriteToCell,     // ,
    PrintCell,       // .
}

/// Const for how large the array is
const PROGRAM_ARRAY_LENGTH: usize = 256;

/// Struct to represent Brainfuck program
pub struct BrainfuckProgram {
    program_array: [u32; PROGRAM_ARRAY_LENGTH],
    instructions: Vec<BrainfuckInstructions>,
    pointer: usize,
    return_address: usize,
}

impl BrainfuckProgram {
    /// Creates a new BrainfuckProgram
    ///
    /// Parameters
    /// instructions:   The list of instructions for the program to run
    pub fn new(instructions: Vec<BrainfuckInstructions>) -> BrainfuckProgram {
        BrainfuckProgram {
            program_array: [0; PROGRAM_ARRAY_LENGTH],
            instructions,
            pointer: 0,
            return_address: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BrainfuckInstructions::{
        BeginLoop,       // [
        DecreasePointer, // <
        DecreaseValue,   // -
        EndLoop,         // ]
        IncreasePointer, // >
        IncreaseValue,   // +
        PrintCell,       // .
        WriteToCell,     // ,
    };
    use super::BrainfuckProgram;

    #[test]
    /// Checks if the constructor produces a BrainfuckProgram with the right instructions
    fn constructor_works() {
        let instructions = vec![
            IncreasePointer,
            DecreasePointer,
            IncreaseValue,
            DecreaseValue,
            BeginLoop,
            EndLoop,
            WriteToCell,
            PrintCell,
        ];

        let brainfuck_program = BrainfuckProgram::new(instructions.clone());

        assert_eq!(brainfuck_program.instructions, instructions)
    }
}
