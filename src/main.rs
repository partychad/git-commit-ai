

mod gpt_api;
mod commands;
fn main() {
    use gpt_api::CommitMessageGenerator;
    use commands::Commands;
    let git_diff = Commands::new("git".to_string(), vec!["diff"]);
    let git_status = Commands::new("git".to_string(), vec!["status", "--short"]);

    let generator = CommitMessageGenerator::new(
        "https://api.openai.com/v1/chat/completions",
        "gpt-4",
        "Based on the following git diff and git status output, suggest a formatted and structured but succinct commit message\n",
        "GPT_API_KEY"
    );

    let commit_msg = generator.generate_commit_message(&git_diff.call(), &git_status.call());
    match commit_msg {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("Error encountered: {:?}", err),
    }


}
