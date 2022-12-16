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
    return_address_vec: Vec<usize>,
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
            return_address_vec: Vec::new(),
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
                // [ Instruction
                BeginLoop => {
                    // Adding the value of this instruction to the vec of return address.
                    // A vec is used so that nested loops work
                    self.return_address_vec.push(self.program_counter.clone());
                }
                // ] Instruction
                EndLoop => {
                    // If the current cell is not = to 0 then loop
                    if self.memory_array[self.pointer] != 0 {
                        // Setting the program counter back to the start of the loop
                        self.program_counter = self.pop_last_return_address();
                        // Skiping the rest of the loop so that the program counter is not
                        // incremented
                        continue;
                    }
                }
                // . Instruction
                // NEED TO TEST THIS, I dont actually know if this works or not
                PrintCell => {
                    print!("{}", self.memory_array[self.pointer] as char);
                }
                _ => (),
            }

            // Increasing the program counter to move onto the next instruction on the next loop
            self.program_counter += 1;
        }
    } 

    /// Pops of the last return address added and returns it
    fn pop_last_return_address(&mut self) -> usize {
        // Getting the index
        let index = self.return_address_vec.len() - 1;

        // Cloning the return address and removing it
        let return_address = self.return_address_vec[index].clone();
        self.return_address_vec.remove(index);

        return_address
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
    /// tests if the pop_last_return_address method works
    fn pop_last_return_address_works() {
        let instructions = Vec::new();

        let mut program = BrainfuckProgram::new(instructions);

        program.return_address_vec.push(1);
        program.return_address_vec.push(2);
        program.return_address_vec.push(4);

        let return_address = program.pop_last_return_address();

        assert_eq!(return_address, 4);
        assert_eq!(vec![1, 2], program.return_address_vec)
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

    #[test]
    /// Tests if looping works
    fn loops_work() {
        // A sum numbers program that uses a loop
        let sum_two_numbers = vec![
            // Setting up the two numbers to add
            IncreaseValue, // 2
            IncreaseValue,
            IncreasePointer,
            IncreaseValue, // 3
            IncreaseValue,
            IncreaseValue,

            BeginLoop,
                DecreaseValue,
                DecreasePointer,
                IncreaseValue,
                IncreasePointer,
            EndLoop
        ];

        // Creating the Brainfuck program and running it
        let mut program = BrainfuckProgram::new(sum_two_numbers);
        program.run();

        // Asserting that the program ran as expected
        assert_eq!(program.memory_array[0], 5);
        assert_eq!(program.memory_array[1], 0)
    }
}
