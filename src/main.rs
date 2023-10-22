mod commands_module {
    use std::fmt;
    use std::fmt::{Formatter, Result};
    use std::process::Command;

    pub struct Commands {
        command: String,
        arguments: String,
    }

    impl Commands {
        pub fn new(command_name: String, arguments: Vec<&str>) -> Commands {
            Commands {
                command: command_name,
                arguments: arguments.join(" "),
            }
        }

        pub fn call(&self) -> String {
            let mut cmd = Command::new(&self.command);

            for arg in self.arguments.split_whitespace(){
                cmd.arg(arg);
            }
            let output = cmd.output()
                .expect("Failed to execute command");

            // Convert the output bytes to a String and print it
            String::from_utf8(output.stdout).expect("Not UTF8")
        }

        pub fn print(&self) {
            let result = self.call();
            println!("{}", result);
        }

        pub fn debug(&self) {
            println!("command : {} \nargument : {}", &self.command, &self.arguments);
        }
    }

    impl fmt::Debug for Commands {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Output of call \"{} {}\" is:\n{}", &self.command , &self.arguments.as_str(), self.call())
        }
    }
}

mod gpt_api;
fn main() {
    use gpt_api::generate_commit_message;
    use commands_module::Commands;
    let ls = Commands::new("ls".to_string(), vec!["-la"]);
    let pwd = Commands::new("pwd".to_string(), vec![]);
    let git_log = Commands::new("git".to_string(), vec!["log", "-3", "--oneline"]);
    let git_diff = Commands::new("git".to_string(), vec!["diff"]);

    let commands = vec![ls,pwd,git_log];

    for command in commands.iter() {
        // println!("{:?}" , command);

    }

    match generate_commit_message(&git_diff.call()) {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error generating commit message: {:?}", e),
    }


}
