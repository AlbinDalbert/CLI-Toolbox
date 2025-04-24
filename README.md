# CLI Toolbox

A library for creating simple and good looking terminal interfaces. Designed primarily for programs with multiple sub-programs.

## Features

- System management with name, color, style, and sleep time
- Program management with run functions and metadata
- Tag-based organization
- Cross-platform shell command execution
- Error handling and input validation

## Quick Start

```rust
use cli_toolbox::{System, Program, TermColor};

fn main() {
    // Create a system with default settings
    let mut system = System::builder("My CLI Tool")
        .use_defaults()
        .build();

    // Add a simple program
    system.add_program_with_inheritance("hello".to_string(), || {
        println!("Hello, world!");
    });

    // Run the menu
    system.menu();
}
```

## Examples

### Basic Usage

See `examples/basic.rs` for a simple example showing:
- System creation
- Program addition
- Shell command execution
- Help display
- Menu navigation

### Advanced Usage

See `examples/advanced.rs` for more complex features:
- Input validation
- Error handling
- Tag-based filtering
- Custom styling
- Program inheritance

## API Overview

### System

The main hub for managing programs and user interaction.

```rust
let system = System::builder("My Tool")
    .use_defaults()
    .color(TermColor::Blue)
    .sleep(100)
    .build();
```

### Program

Individual programs that can be run from the system.

```rust
let program = Program::builder("My Program")
    .use_defaults()
    .description("A description")
    .tag("example")
    .action(|| {
        // Program logic here
    })
    .build();
```

### Features

- **Builder Pattern**: Clean configuration of systems and programs
- **Tag System**: Organize programs with multiple tags
- **Cross-Platform**: Works on both Windows and Unix systems
- **Error Handling**: Proper error types and handling
- **Input Validation**: Built-in validation support
- **Styling**: Customizable colors and formatting

## License

This project is licensed under the Apache-2.0 License - see the LICENSE file for details.

## About
The Idea with this library is to have a easy to use framework to easily make terminal programs with multiple sub-programs. 
A way to make the text and color for the prints to make sense for the particular part of the program.

The Program is used to handle the interface of a program. While the System is used as a navigation point consisting of other programs.
Working more as a Map.
## Include It
To use this crate. Under the dependencies in your Cargo.toml, just add this crate as followed

```cli_toolbox = { git = "https://github.com/AlbinDalbert/CLI-Toolbox.git" }```

Then just update your cargo the everything is up and running.

## Example
For example on how to use this, take a peek here https://github.com/AlbinDalbert/advent-of-code-2022

## Road-map
This is still in a very early stage, a lot of things have not been implemented and/or works yet.

### Core Functions
- [x]    Customizable for each instance of a `System` and `Program`
- [x]    Program inherits optional attributes from `System` by default
- [x]    Programs can be executed via the `run()` function, (e.g. They are fundamentally linked together, not just for looks)
- [x]    Change the cargo name to something better and more telling
- [x]    Make a generic `Menu` where each item has a `name` and `action`(function)

### Optional Steps
- [x]   Benchmark, runs all programs in a system and times them.
- [ ]   make the "help" command have auto-generated information about systems, programs and commands
- [ ]   System can have sub-system
