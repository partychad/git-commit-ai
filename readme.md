Certainly! Here's a draft for your README:

---

# Automatic Commit Message Generator with ChatGPT

Generate insightful and structured commit messages with the power of OpenAI's ChatGPT.

## Overview

This repository provides a Rust utility that takes in a git diff as input and uses ChatGPT to suggest a well-structured commit message. Say goodbye to cryptic or hastily written commit messages!

## Features

- Leverages the linguistic capabilities of ChatGPT.
- Supports extraction of API key from environment variables for secure usage.
- Handles potential errors gracefully with comprehensive messages.
- Highly configurable message generation.

## Prerequisites

- Rust (latest stable version recommended).
- An API key from OpenAI with access to the ChatGPT model.

## Setup & Usage

1. **Clone the repository:**

```bash
git clone git@github.com:partychad/git-commit-ai.git
cd git-commit-ai
```

2. **Set up the OpenAI API key:**

Before running the application, you need to set up your OpenAI API key. You can do this using environment variables:

```bash
export GPT_API_KEY={API_KEY}
```

3. **Run the application:**

Compile and run the application with:

```bash
cargo run
```

Provide the git diff input and wait for the generated commit message.

4. **Customization:**

You can customize the default behavior, endpoint, model, and templated message by modifying the `CommitMessageGenerator` instantiation in the main program.

## Contributing

Feel free to open issues or pull requests if you'd like to improve the project. All contributions are welcome!

## License

This project is licensed under the MIT License.


---

