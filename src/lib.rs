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
const MEMORY_ARRAY_LENGTH: usize = 256;

/// Struct to represent Brainfuck program
pub struct BrainfuckProgram {
    /// The array of memory that the program modifies
    memory_array: [u32; MEMORY_ARRAY_LENGTH],

    /// The instructions for the program to run
    instructions: Vec<BrainfuckInstructions>,

    /// The pointer contains the index of the current cell in the memory array
    pointer: usize,

    /// The index of the current instruction in the instructions vec
    program_counter: usize,

    /// The return address for storing the instruction that the loop begins in
    return_address: usize,
}

impl BrainfuckProgram {
    /// Creates a new BrainfuckProgram
    ///
    /// Parameters
    /// instructions:   The list of instructions for the program to run
    pub fn new(instructions: Vec<BrainfuckInstructions>) -> BrainfuckProgram {
        BrainfuckProgram {
            memory_array: [0; MEMORY_ARRAY_LENGTH],
            instructions,
            pointer: 0,
            program_counter: 0,
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
