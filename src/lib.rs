//! A Brainfuck (or Brainf*ck) interpreter based on the specifications written in:
//! https://github.com/brain-lang/brainfuck/blob/master/brainfuck.md

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
        }
    }

    /// Runs the brainfuck program
    pub fn run(&mut self, debug: bool) {
        if debug {
            println!("{:#?}", &self.instructions);
        }

        // This will run the program until the end of the instructions is run. We can't just loop
        // through them as loops will not work
        while self.program_counter != self.instructions.len() {
            match self.instructions[self.program_counter] {
                // > Instruction
                IncreasePointer => {
                    if self.pointer != (MEMORY_ARRAY_LENGTH - 1) {
                        self.pointer += 1;
                    } else if debug {
                        println!("ERROR, cannot increase pointer! PC: {}", self.program_counter);
                    }
                }

                // < Instruction
                DecreasePointer => {
                    if self.pointer != 0 {
                        self.pointer -= 1;
                    } else if debug {
                        println!("ERROR, cannot decrease pointer! PC: {}", self.program_counter);
                    }
                }

                // + Instruction
                IncreaseValue => {
                    if self.memory_array[self.pointer] == u8::MAX {
                        // Simulate an overflow if the cell's current value is 255
                        self.memory_array[self.pointer] = 0;
                    } else {
                        // Else just increment the value
                        self.memory_array[self.pointer] += 1;
                    }
                }

                // - Instruction
                DecreaseValue => {
                    if self.memory_array[self.pointer] == 0 {
                        // Simulate an overflow if the cell's current value is 0
                        self.memory_array[self.pointer] = u8::MAX;
                    } else {
                        // Else just increment the value
                        self.memory_array[self.pointer] -= 1;
                    }
                }

                // [ Instruction
                BeginLoop => {
                    // Jump to the instruction after the matching ] if the current cell is zero
                    if self.memory_array[self.pointer] == 0 {
                        // By setting the pc to the location of the next ] the program counter will
                        // increment at the end of this match statment to the instruction after it
                        self.program_counter = self.find_end_loop().unwrap();
                        if debug {
                            println!("Jumped!");
                        }
                        continue;
                    }
                    println!("Begin Loop!");
                }

                // ] Instruction
                EndLoop => {
                    // Jump if the current cell is not equal to zero and skip to the next iteration
                    // of the loop
                    if self.memory_array[self.pointer] != 0 {
                        self.program_counter = self.find_begin_loop().unwrap();
                        if debug {
                            println!("Looped!");
                        }
                        continue;
                    }
                }

                // . Instruction
                // NEED TO TEST THIS, I dont actually know if this works or not
                PrintCell => {
                    let num = self.memory_array[self.pointer];
                    print!("{}", num as char)
                }

                WriteToCell => (),
            }

            if debug {
                self.print_memory_array();
            }

            // Increasing the program counter to move onto the next instruction on the next loop
            self.program_counter += 1;
        }

        if debug {
            self.print_memory_array();
        }
    }

    /// Finds the closest ] instruction to the program counter
    fn find_end_loop(&self) -> Option<usize> {
        // Looping through all the instructions
        for i in self.program_counter..self.instructions.len() {
            match self.instructions[i] {
                EndLoop => return Some(i),
                _ => continue,
            }
        }

        None
    }

    /// Finds the closest [ instruction above the program_counter
    fn find_begin_loop(&self) -> Option<usize> {
        // Looping through all the instructions
        for i in (0..self.program_counter).rev() {
            match self.instructions[i] {
                BeginLoop => return Some(i),
                _ => continue,
            }
        }

        None
    }

    pub fn print_memory_array(&self) {
        println!();
        for i in 0..16 {
            if i == self.pointer {
                print!(">{}, ", self.memory_array[i]);
                continue;
            }

            print!("{}, ", self.memory_array[i]);
        }
        println!();
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
    /// Tests if the find_end_loop method works
    fn find_end_loop_works() {
        let instructions = vec![
            BeginLoop,
            IncreaseValue,
            IncreasePointer,
            EndLoop,
            IncreaseValue,
        ];

        let program = BrainfuckProgram::new(instructions);

        assert_eq!(program.find_end_loop().unwrap(), 3);
    }

    #[test]
    /// Tests if the find_end_loop method works with nested loops
    fn find_end_loop_nested_works() {
        let instructions = vec![
            BeginLoop,
            IncreaseValue,
            BeginLoop,
            IncreasePointer,
            IncreaseValue,
            EndLoop,
            EndLoop,
            IncreaseValue,
        ];

        let mut program = BrainfuckProgram::new(instructions);

        program.program_counter = 2;

        assert_eq!(program.find_end_loop().unwrap(), 5);

        // See find_begin_loop_works for why this currently fails
        program.program_counter = 0;

        assert_eq!(program.find_end_loop().unwrap(), 6);
    }

    #[test]
    /// Tests if the find_begin_loop method works
    fn find_begin_loop_works() {
        // testing with the begin loop at the start
        let instructions = vec![
            BeginLoop,
            IncreaseValue,
            IncreasePointer,
            EndLoop,
            IncreaseValue,
        ];

        let mut program = BrainfuckProgram::new(instructions);

        program.program_counter = 3;

        assert_eq!(program.find_begin_loop().unwrap(), 0);
    }

    #[test]
    /// Tests if the find_end_loop method works with nested loops
    fn find_begin_loop_nested_works() {
        let instructions = vec![
            BeginLoop,
            IncreaseValue,
            BeginLoop,
            IncreasePointer,
            IncreaseValue,
            EndLoop,
            EndLoop,
            IncreaseValue,
        ];

        let mut program = BrainfuckProgram::new(instructions);

        program.program_counter = 5;

        assert_eq!(program.find_begin_loop().unwrap(), 2);

        // This has been the problem all along... this method doesn't account for if there are loop
        // begginings before the one this one closes...
        program.program_counter = 6;

        assert_eq!(program.find_begin_loop().unwrap(), 0);
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
            IncreaseValue,
        ];

        // Creating the Brainfuck program and running it
        let mut program = BrainfuckProgram::new(instructions);
        program.run(false);

        // Writing the expected state of the memory array
        let mut expected_array: [u8; MEMORY_ARRAY_LENGTH] = [0; MEMORY_ARRAY_LENGTH];
        expected_array[1] = 2;
        expected_array[2] = 1;

        // Testing if they are the same
        assert!(expected_array.iter().eq(program.memory_array.iter()))
    }

    #[test]
    fn overflow_works() {
        // Testing 0 - 1 = 255
        let instructions = vec![DecreaseValue];

        let mut program = BrainfuckProgram::new(instructions);

        program.run(false);

        assert_eq!(program.memory_array[0], 255);

        // Tesing 255 + 1 = 0
        let instructions = vec![
            // Setting the current cell to 16
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            // Entering the loop
            BeginLoop,
            // Adding second to the next cell
            IncreasePointer,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            IncreaseValue,
            DecreasePointer,
            DecreaseValue,
            EndLoop,
        ];

        let mut program = BrainfuckProgram::new(instructions);

        program.run(false);

        assert_eq!(program.memory_array[1], 0)
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
            EndLoop,
        ];

        // Creating the Brainfuck program and running it
        let mut program = BrainfuckProgram::new(sum_two_numbers);
        program.run(false);

        // Asserting that the program ran as expected
        assert_eq!(program.memory_array[0], 5);
        assert_eq!(program.memory_array[1], 0)
    }

    #[test]
    /// Tests if BeginLoop jumpts
    fn begin_loop_jumps_if_cell_is_zero() {
        let instructions = vec![
            BeginLoop,
            // This will create an infinite loop if this loop is not skipped
            IncreaseValue,
            EndLoop,
            DecreaseValue,
        ];

        let mut program = BrainfuckProgram::new(instructions);
        program.run(false);

        assert_eq!(program.memory_array[0], 255);
    }
}
