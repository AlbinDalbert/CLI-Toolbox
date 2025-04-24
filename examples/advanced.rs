use cli_toolbox::{System, Program, TermColor};
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

    // Add all programs to the system
    system.append_program(validate_program);
    system.append_program(calculator_program);
    system.append_program(tagged_program);

    // Show all available tags
    println!("Available tags: {:?}", system.all_tags());

    // Show programs with a specific tag
    system.show_tagged_menu("example");

    // Run the main menu
    system.menu();
} 