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
const MEMORY_ARRAY_LENGTH: usize = 30000;

/// Struct to represent Brainfuck program
pub struct BrainfuckProgram {
    /// The array of memory that the program modifies
    memory_array: [u8; MEMORY_ARRAY_LENGTH],

    /// The instructions for the program to run
    instructions: Vec<BrainfuckInstructions>,

    /// The pointer contains the index of the current cell in the memory array
    pointer: usize,

    /// The index of the current instruction in the instructions vec
    program_counter: usize,

    /// The return address for storing the instruction that the loop begins in
    return_address: usize,
}

use BrainfuckInstructions::*;

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

    /// Runs the brainfuck program
    pub fn run(&mut self) {
        // This will run the program until the end of the instructions is run. We can't just loop
        // through them as loops will not work
        while self.program_counter != self.instructions.len() {
            match &self.instructions[self.program_counter] {
                // > Instruction
                IncreasePointer => {
                    if self.pointer < (MEMORY_ARRAY_LENGTH - 1) {
                        self.pointer += 1;
                    }
                },
                // < Instruction
                DecreasePointer => {
                    if self.pointer != 0 {
                        self.pointer -= 1;
                    }
                },
                // + Instruction
                IncreaseValue => {
                    if self.memory_array[self.pointer] != u8::MAX {
                        self.memory_array[self.pointer] += 1;
                    }
                }
                // - Instruction
                DecreaseValue => {
                    if self.memory_array[self.pointer] != 0 {
                        self.memory_array[self.pointer] -= 1;
                    }
                }
                _ => (),
            }

            // Increasing the program counter to move onto the next instruction on the next loop
            self.program_counter += 1;
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

    use super::MEMORY_ARRAY_LENGTH;

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

    #[test]
    /// Tests if IncreasePointer, DecreasePointer, IncreaseValue, and DecreaseValue Instructions 
    /// works
    fn first_four_instructions_works() {
        // The instruction set to test
        let instructions = vec![
            IncreaseValue,
            IncreasePointer,
            IncreaseValue,
            IncreaseValue,
            DecreasePointer,
            DecreaseValue,
            IncreasePointer,
            IncreasePointer,
            IncreaseValue
        ];

        // Creating the Brainfuck program and running it
        let mut program = BrainfuckProgram::new(instructions);
        program.run();

        // Writing the expected state of the memory array
        let mut expected_array: [u8; MEMORY_ARRAY_LENGTH] = [0; MEMORY_ARRAY_LENGTH];
        expected_array[1] = 2;
        expected_array[2] = 1;

        // Testing if they are the same
        assert!(expected_array.iter().eq(program.memory_array.iter()))
    }
}
