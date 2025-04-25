use cli_toolbox::{System, Program};

fn main() {
    // Create a system with default settings
    let mut system = System::builder("My CLI Tool")
        .use_defaults()
        .build();

    // Add a simple program
    system.add_program_with_inheritance("hello".to_string(), || {
        println!("Hello, world!");
    });

    // Add a program with custom settings
    let program = Program::builder("greet")
        .use_defaults()
        .description("A friendly greeting program")
        .tag("greeting")
        .tag("example")
        .action(|| {
            println!("Welcome to the CLI Toolbox!");
        })
        .build();
    system.append_program(program);

    // Add a program with shell command (cross-platform)
    let program = Program::builder("list")
        .use_defaults()
        .description("List current directory")
        .tag("system")
        .shell_command(if cfg!(windows) {
            "dir"
        } else {
            "ls -l"
        })
        .build();
    system.append_program(program);

    // Show help
    system.show_help();

    // Run the menu
    system.menu();
} 