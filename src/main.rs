mod commands;
mod gpt_api;
use clap::{App, SubCommand};
use commands::Commands;
use gpt_api::CommitMessageGenerator;
fn main() {
    let matches = App::new("Git GPT Helper")
        .version("1.0")
        .author("<partychad@protonmail.com>")
        .about("Automates git commit messages using GPT suggestions.")
        .subcommand(SubCommand::with_name("s").about("Displays git status"))
        .subcommand(
            SubCommand::with_name("d")
                .about("Display the generated commit message without committing"),
        )
        .subcommand(SubCommand::with_name("config").about("Displays the configuration parameters"))
        .subcommand(SubCommand::with_name("c").about("Commits the changes"))
        .subcommand(SubCommand::with_name("p").about("Pushes the changes (add + commit + push)"))
        .get_matches();

    // Check which subcommand (if any) was used
    match matches.subcommand_name() {
        Some("s") => display_status(),
        Some("d") => {
            display_commit_message();
        }
        Some("config") => display_config(),
        Some("c") => commit(),
        Some("p") => push(),
        None => println!("No subcommand was used"),
        _ => unreachable!(), // If someone added a subcommand but didn't add a case here
    }
}
fn display_status() {
    let git_status = Commands::new("git".to_string(), vec!["status".to_string()]);
    let status = git_status.call();
    println!("Status {}", status);
}

fn display_commit_message() -> String {
    let generator = CommitMessageGenerator::new(
    "https://api.openai.com/v1/chat/completions",
    "gpt-4",
    "Based on the following git diff and git status output, suggest a formatted and structured but succinct commit message\n",
    "GPT_API_KEY"
    );
    let git_diff = Commands::new("git".to_string(), vec!["diff".to_string()]);
    let git_status = Commands::new("git".to_string(), vec!["status".to_string()]);
    let commit_msg = generator.generate_commit_message(&git_diff.call(), &git_status.call());
    let final_msg = match commit_msg {
        Ok(msg) => {
            msg
        }
        Err(err) => {
            eprintln!("Error encountered: {}", err);
            return String::new();
        }
    };
    println!("Commit Message : {}", final_msg);
    final_msg
}

fn display_config() {
    let generator = CommitMessageGenerator::new(
        "https://api.openai.com/v1/chat/completions",
        "gpt-4",
        "Based on the following git diff and git status output, suggest a formatted and structured but succinct commit message\n",
        "GPT_API_KEY"
    );
    generator.display_parameters();
}

fn commit() {
    let commit_msg = display_commit_message();
    if !commit_msg.is_empty() {
        let git_commit = Commands::new(
            "git".to_string(),
            vec!["commit".to_string(), "-m".to_string(), commit_msg],
        );
        let git_add = Commands::new("git".to_string(), vec!["add".to_string(), ".".to_string()]);
        git_add.call();
        git_commit.call();
    } else {
        eprintln!("Error: Commit message is empty!");
    }
}

fn push() {
    commit();
    let git_push = Commands::new(
        "git".to_string(),
        vec!["push".to_string(), "--force-with-lease".to_string()],
    );
    git_push.call();
}
