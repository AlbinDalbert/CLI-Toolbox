//! CLI Toolbox core functionality
//! 
//! This module contains the main components for building CLI applications:
//! 
//! # Core Components
//! 
//! - [`System`]: The main CLI manager that handles program organization and user interaction
//! - [`Program`]: Individual commands or sub-programs that can be executed
//! - [`ShellCommand`]: Safe cross-platform shell command execution
//! 
//! # Error Handling
//! 
//! The library uses [`CliError`] for error handling, providing specific error types for:
//! - Invalid input
//! - Command failures
//! - Program not found errors
//! 
//! # Example
//! 
//! ```rust
//! use cli_toolbox::{System, Program, TermColor};
//! 
//! let mut system = System::builder("My Tool")
//!     .use_defaults()
//!     .build();
//! 
//! // Add a program with custom settings
//! let program = Program::builder("greet")
//!     .description("A friendly greeting")
//!     .action(|| println!("Hello!"))
//!     .build();
//! 
//! system.append_program(program);
//! ```

pub mod system;
pub mod program;

pub use system::System;
pub use program::Program; 