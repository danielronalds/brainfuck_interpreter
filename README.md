# Brainfuck Interpreter

An interpreter for the Brainfuck programming language that is written in Rust, because why not

## Installation

### Installing with Cargo

Installing using cargo is the easiest option, as you only need to run the following command

```
cargo install --git https://github.com/danielronalds/brainfuck_interpreter.git
```

### Building from source

Alternatively you can build a binary from source by running the following commands

```
git clone https://github.com/danielronalds/brainfuck_interpreter.git
cd brainfuck_interpreter/
cargo build
```

## Using the interpreter

To run a brainfuck program simply run the program with the path to the file. In the following 
example the brainfuck program we'll run is called if_else.bf, located in our current directory
```
brainfuck_interpreter if_else.bf
```

### Debugging

To produce a log of the brainfuck program, run the program using the debug flag. This prints the 
state of the memory tape after every instruction to the console, as well as when a loop is run. 
Using the previous example, running the program would use the following command

```
brainfuck_interpreter --debug if_else.bf
```
The placement of the flag does not matter, so the following also works

```
brainfuck_interpreter if_else.bf --debug
```

This by itself is not particularly useful, however the output can be written to a file like so
```
brainfuck_interpreter if_else.bf --debug > bf_log.txt
```

