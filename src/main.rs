

mod gpt_api;
mod commands;
fn main() {
    use gpt_api::CommitMessageGenerator;
    use commands::Commands;
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
