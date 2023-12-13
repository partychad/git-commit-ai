mod commands;
mod cursor;
mod gpt_api;
use clap::{App, SubCommand};
use colored::Colorize;
use commands::Commands;
use gpt_api::CommitMessageGenerator;
use std::io::{self, Read};

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
const API_KEY: &str = "GPT_API_KEY";
const AI_MODEL: &str = "gpt-4";
const PROMPT: &str = "Based on the following git diff and git status, write a short commit message and use a gitmoji. Only include the description without any titles or new lines. Dont forget to include newly added files with their name";

fn main() {
    let matches = App::new("Git GPT Helper")
        .version("1.0")
        .author("<partychad@protonmail.com>")
        .about("Automates git commit messages using GPT suggestions.")
        .subcommand(SubCommand::with_name("s").about("Interactive promt to commit & push of selected files"))
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
        Some("s") => selected_commit(),
        Some("d") => {
            display_commit_message(None,None);
        }
        Some("config") => display_config(),
        Some("c") => commit(None,None),
        Some("p") => push(None,None),
        None => println!("No subcommand was used"),
        _ => unreachable!(), // If someone added a subcommand but didn't add a case here
    }
}
fn selected_commit() {
    let git_status = Commands::new("git".to_string(), vec!["status".to_string()]);
    let status = git_status.call();
    let (_, untracked, modified) = parse_git_status(&status);
    let mut combined = Vec::new();
    println!("{}", "Git Status:".green());
    combined.extend(untracked);
    combined.extend(modified.iter().cloned());
    combined.push("Done".to_string());
    let selection = cursor::navigate_strings(&combined);
    match selection {
        Some(files) => {
            let diff_arguments: Vec<String> =  files // Truncating the newly added files
                .iter()
                .filter(|file| modified.contains(file))
                .cloned()
                .collect();
            push(Some(files),Some(diff_arguments))
        }
        None => {
            println!("No files selected. Terminating...\n");
        }
    }
}

fn display_commit_message(status:Option<Vec<String>>, diff:Option<Vec<String>>) -> String {
    let generator = CommitMessageGenerator::new(API_URL, AI_MODEL, PROMPT, API_KEY);
    let mut diff_arguments = vec!["diff".to_string()];
    if let Some(diff) = diff {
        diff_arguments.extend(diff);
    }

    let mut status_arguments = vec!["status".to_string()];
    if let Some(status) = status {
        status_arguments.extend(status);
    }

    let git_diff = Commands::new("git".to_string(), diff_arguments);
    let git_status = Commands::new("git".to_string(), status_arguments);
    let commit_msg = generator.generate_commit_message(&git_diff.call(), &git_status.call());
    let final_msg = match commit_msg {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("{} {}\n", "Error encountered:".red(), err);
            return String::new();
        }
    };
    print_commit_metadata(parse_git_status(&git_status.call()), &final_msg);

    final_msg
}

fn display_config() {
    let generator = CommitMessageGenerator::new(API_URL, AI_MODEL, PROMPT, API_KEY);
    generator.display_parameters();
}

fn commit(status:Option<Vec<String>>, diff:Option<Vec<String>>) {
    let commit_msg = display_commit_message(status.clone(), diff);
    if !commit_msg.is_empty() {
        let git_commit = Commands::new(
            "git".to_string(),
            vec!["commit".to_string(), "-m".to_string(), commit_msg],
        );
        let mut add_arguments = vec!["add".to_string()];
        if let Some(status) = status {
            add_arguments.extend(status);
        } else {
            add_arguments.push(".".to_string());
        }
        press_enter_to_continue();
        let git_add = Commands::new("git".to_string(), add_arguments);
        git_add.call();
        let output = git_commit.call();
        println!("{}", output);
    } else {
        eprintln!("Error: Commit message is empty!");
    }
}

fn push(status:Option<Vec<String>>, diff:Option<Vec<String>>) {
    commit(status, diff);
    let git_push = Commands::new(
        "git".to_string(),
        vec!["push".to_string(), "--force-with-lease".to_string()],
    );
    let output: String = git_push.call();
    println!("{}", output);
}

fn parse_git_status(output: &str) -> (String, Vec<String>, Vec<String>) {
    let mut branch_name = String::new();
    let mut untracked_files = Vec::new();
    let mut modified_files = Vec::new();

    let lines: Vec<&str> = output.lines().collect();
    let mut skip_line = 0;
    let mut section = "";

    for line in lines {
        if skip_line > 0 {
            skip_line -= 1;
            continue;
        }
        if line.starts_with("On branch") {
            branch_name = line.replace("On branch ", "");
        } else if line.starts_with("Changes not staged for commit:") {
            skip_line = 2;
            section = "modified";
        } else if line.starts_with("Untracked files:") {
            skip_line = 1;
            section = "untracked";
        } else if !section.is_empty() {
            if line.is_empty() {
                section = "";
            }
            match section {
                "modified" => modified_files.push(line.trim().replace("modified:   ", "")),
                "untracked" => untracked_files.push(line.trim().to_string()),
                _ => {}
            }
        }
    }

    (branch_name, untracked_files, modified_files)
}

fn print_commit_metadata(data: (String, Vec<String>, Vec<String>), commit_msg: &str) {
    let (branch, untracked, modified) = data;
    println!("{} {}", "Branch:".green(), branch);
    if !untracked.is_empty() {
        println!("{}", "Untracked Files:".green());
        for file in untracked {
            println!("\t{}", file);
        }
    }
    if !modified.is_empty() {
        println!("{}", "Modified Files:".green());
        for file in modified {
            println!("\t{}", file);
        }
    }
    if !commit_msg.is_empty() {
        println!("{} {}\n", "Commit Message:".green(), commit_msg);
    }
}

fn press_enter_to_continue() {
    println!("Press Enter To Commit and Push...");

    // Lock stdin so we can read from it
    let stdin = io::stdin();
    let mut iterator = stdin.lock().bytes();

    // Wait for a single byte to be pressed and then exit
    iterator.next();
}
