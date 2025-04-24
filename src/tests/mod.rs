#[cfg(test)]
mod tests {
    use crate::{System, Program, TermColor};
    use crate::cli::system::CliError;
    use std::io::{self, Write};
    use std::sync::{Mutex, Arc};
    use std::sync::atomic::{AtomicBool, Ordering};

    // Mock for capturing stdout
    struct MockStdout {
        output: Mutex<Vec<u8>>,
    }

    impl MockStdout {
        fn new() -> Self {
            Self {
                output: Mutex::new(Vec::new()),
            }
        }

        fn get_output(&self) -> String {
            let output = self.output.lock().unwrap();
            String::from_utf8_lossy(&output).to_string()
        }
    }

    impl Write for MockStdout {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_system_builder() {
        let system = System::builder("Test System")
            .use_defaults()
            .build();
        
        assert_eq!(system.name(), "Test System");
        assert_eq!(system.get_sleep(), 100);
        assert!(!system.get_silence());
    }

    #[test]
    fn test_program_builder() {
        let program = Program::builder("Test Program")
            .use_defaults()
            .description("A test program")
            .tag("test")
            .action(|| {})
            .build();
        
        assert_eq!(program.name(), "Test Program");
        assert_eq!(program.description(), "A test program");
        assert!(program.has_tag("test"));
    }

    #[test]
    fn test_tag_system() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .build();

        let prog1 = Program::builder("prog1")
            .use_defaults()
            .tag("test")
            .tag("example")
            .action(|| {})
            .build();
        
        let prog2 = Program::builder("prog2")
            .use_defaults()
            .tag("test")
            .action(|| {})
            .build();

        system.append_program(prog1);
        system.append_program(prog2);

        let tagged = system.programs_with_tag("test");
        assert_eq!(tagged.len(), 2);

        let all_tags = system.all_tags();
        assert_eq!(all_tags.len(), 2);
        assert!(all_tags.contains(&"test".to_string()));
        assert!(all_tags.contains(&"example".to_string()));
    }

    #[test]
    fn test_error_handling() {
        let system = System::builder("Test System")
            .use_defaults()
            .build();

        // Test program not found
        let result = system.run_program(999);
        assert!(matches!(result, Err(CliError::ProgramNotFound(_))));
    }

    #[test]
    fn test_program_inheritance() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .color(TermColor::Blue)
            .sleep(200)
            .silent(true)
            .build();

        system.add_program_with_inheritance("test".to_string(), || {});
        
        let program = &system.programs()[0];
        assert_eq!(program.get_sleep(), 200);
        assert!(program.get_silence());
    }

    #[test]
    fn test_program_execution() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        
        let program = Program::builder("Test Program")
            .use_defaults()
            .action(move || {
                executed_clone.store(true, Ordering::SeqCst);
            })
            .build();

        program.run();
        assert!(executed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_system_help() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .build();

        let program = Program::builder("Test Program")
            .use_defaults()
            .description("A test program")
            .tag("test")
            .action(|| {})
            .build();

        system.append_program(program);
        system.show_help(); // This will print to stdout, but at least we test it doesn't crash
    }

    #[test]
    fn test_system_print() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .build();

        system.print("Test message"); // This will print to stdout, but at least we test it doesn't crash
    }

    #[test]
    fn test_system_err() {
        let system = System::builder("Test System")
            .use_defaults()
            .build();

        system.err(Some(&"Test error".to_string())); // This will print to stdout, but at least we test it doesn't crash
    }

    #[test]
    fn test_program_print() {
        let program = Program::builder("Test Program")
            .use_defaults()
            .action(|| {})
            .build();

        program.print("Test message"); // This will print to stdout, but at least we test it doesn't crash
    }

    #[test]
    fn test_program_err() {
        let program = Program::builder("Test Program")
            .use_defaults()
            .action(|| {})
            .build();

        program.err("Test error"); // This will print to stdout, but at least we test it doesn't crash
    }

    #[test]
    fn test_empty_program_list() {
        let system = System::builder("Test System")
            .use_defaults()
            .build();

        assert_eq!(system.programs().len(), 0);
        let tagged = system.programs_with_tag("test");
        assert_eq!(tagged.len(), 0);
        let all_tags = system.all_tags();
        assert_eq!(all_tags.len(), 0);
    }

    #[test]
    fn test_duplicate_tags() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .build();

        let program = Program::builder("Test Program")
            .use_defaults()
            .tag("test")
            .tag("test") // Duplicate tag
            .action(|| {})
            .build();

        system.append_program(program);
        let all_tags = system.all_tags();
        assert_eq!(all_tags.len(), 1); // Should only have one unique tag
        assert!(all_tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_program_name_collision() {
        let mut system = System::builder("Test System")
            .use_defaults()
            .build();

        let prog1 = Program::builder("test")
            .use_defaults()
            .action(|| {})
            .build();

        let prog2 = Program::builder("test") // Same name
            .use_defaults()
            .action(|| {})
            .build();

        system.append_program(prog1);
        system.append_program(prog2); // This should work, programs can have the same name

        assert_eq!(system.programs().len(), 2);
    }
}

// Integration tests will be added in a separate module when we have proper mocking
// for input and command execution 