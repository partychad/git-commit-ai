use std::fmt;
use std::fmt::{Formatter, Result};
use std::process::Command;

pub struct Commands {
    command: String,
    arguments: Vec<String>,
}

impl Commands {
    pub fn new(command_name: String, arguments: Vec<String>) -> Commands {
        Commands {
            command: command_name,
            arguments,
        }
    }

    pub fn call(&self) -> String {
        let mut cmd = Command::new(&self.command);

        for arg in &self.arguments {
            cmd.arg(arg);
        }

        let output = cmd.output().expect("Failed to execute command");
        // Convert the output bytes to a String and print it
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        format!("{}\n{}", stdout, stderr)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let result = self.call();
        println!("{}", result);
    }
    #[allow(dead_code)]
    pub fn debug(&self) {
        println!(
            "command : {} \nargument : {:?}",
            &self.command, &self.arguments
        );
    }
}
    // Check which subcommand (if any) was used

impl fmt::Debug for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Output of call \"{} {:?}\" is:\n{}",
            &self.command,
            &self.arguments,
            self.call()
        )
    }
}
