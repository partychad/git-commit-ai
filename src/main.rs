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
    use gpt_api::CommitMessageGenerator;
    use commands_module::Commands;
    let git_diff = Commands::new("git".to_string(), vec!["diff"]);

    let generator = CommitMessageGenerator::new(
        "https://api.openai.com/v1/chat/completions",
        "gpt-3.5-turbo",
        "Based on the following git diff, suggest a formatted and structured but succinct commit message\n{}",
    );

    let commit_msg = generator.generate_commit_message(&git_diff.call());
    match commit_msg {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("Error encountered: {:?}", err),
    }


}
