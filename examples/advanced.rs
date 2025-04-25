use cli_toolbox::{System, Program, TermColor, ShellCommand};
#[macro_use] extern crate text_io;

fn main() {
    // Create a system with custom settings
    let mut system = System::builder("Advanced CLI Tool")
        .use_defaults()
        .color(TermColor::Blue)
        .sleep(50)
        .build();

    // Create all programs first
    let validate_program = Program::builder("validate")
        .use_defaults()
        .description("Validate numeric input")
        .tag("example")
        .action(|| {
            println!("Enter a number to validate:");
            let input: String = read!("{}\n");
            match input.trim().parse::<i32>() {
                Ok(num) => println!("Valid number: {}", num),
                Err(_) => println!("Invalid number! Please enter a valid integer."),
            }
        })
        .build();

    let calculator_program = Program::builder("calculator")
        .use_defaults()
        .description("Simple calculator (demonstrates error handling)")
        .tag("example")
        .action(|| {
            println!("Enter first number:");
            let num1: String = read!("{}\n");
            println!("Enter second number:");
            let num2: String = read!("{}\n");
            
            match (num1.trim().parse::<i32>(), num2.trim().parse::<i32>()) {
                (Ok(n1), Ok(n2)) => {
                    println!("Sum: {}", n1 + n2);
                    println!("Difference: {}", n1 - n2);
                    println!("Product: {}", n1 * n2);
                    if n2 != 0 {
                        println!("Quotient: {}", n1 as f32 / n2 as f32);
                    } else {
                        println!("Cannot divide by zero!");
                    }
                },
                _ => println!("Invalid input! Please enter valid integers."),
            }
        })
        .build();

    let tagged_program = Program::builder("tagged-program")
        .use_defaults()
        .description("A program with multiple tags")
        .tag("example")
        .tag("demo")
        .tag("filter")
        .action(|| {
            println!("This program has multiple tags!");
        })
        .build();

    // Add a dynamic shell command example
    let dynamic_shell = Program::builder("dynamic-shell")
        .use_defaults()
        .description("Demonstrates dynamic shell command arguments with validation")
        .tag("example")
        .action(|| {
            println!("Enter command to execute:");
            let cmd: String = read!("{}\n");
            let mut command = ShellCommand::new(cmd.trim());
            
            // Add arguments dynamically based on user input
            println!("Enter arguments (one per line, empty line to finish):");
            loop {
                let input: String = read!("{}\n");
                if input.trim().is_empty() {
                    break;
                }
                command.add_arg(input.trim());
            }

            // Execute with validation
            match command.execute() {
                Ok(status) => {
                    if status.success() {
                        println!("Command executed successfully!");
                    } else {
                        println!("Command failed with status: {}", status);
                    }
                }
                Err(e) => println!("Command error: {:?}", e),
            }
        })
        .build();

    // Add a shell command without validation example
    let unsafe_shell = Program::builder("unsafe-shell")
        .use_defaults()
        .description("Demonstrates shell command without validation")
        .tag("example")
        .action(|| {
            println!("Enter command to execute (no validation):");
            let cmd: String = read!("{}\n");
            let mut command = ShellCommand::new(cmd.trim())
                .with_validation(false);
            
            // Add arguments dynamically based on user input
            println!("Enter arguments (one per line, empty line to finish):");
            loop {
                let input: String = read!("{}\n");
                if input.trim().is_empty() {
                    break;
                }
                command.add_arg(input.trim());
            }

            // Execute without validation
            match command.execute() {
                Ok(status) => {
                    if status.success() {
                        println!("Command executed successfully!");
                    } else {
                        println!("Command failed with status: {}", status);
                    }
                }
                Err(e) => println!("Command error: {:?}", e),
            }
        })
        .build();

    // Add all programs to the system
    system.append_program(validate_program);
    system.append_program(calculator_program);
    system.append_program(tagged_program);
    system.append_program(dynamic_shell);
    system.append_program(unsafe_shell);

    // Show all available tags
    println!("Available tags: {:?}", system.all_tags());

    // Show programs with a specific tag
    system.display();
    system.menu();
    system.menu_with_tags_filter(vec!["demo".to_string()]);

    // Run the main menu
} 