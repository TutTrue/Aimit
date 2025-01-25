# AI Commit Message Generator (aimit)

`aimit` is a command-line tool designed to automate the creation of commit messages using AI models. It analyzes the staged changes in your Git repository and generates a concise and meaningful commit message based on the changes.

## Features

- **AI-Powered Commit Messages**: Uses AI models (currently supports Gemini) to generate commit messages.
- **Customizable Prompt**: Allows you to customize the prompt used to generate commit messages.
- **Multiple AI Models**: Supports multiple AI models (currently only Gemini is implemented).
- **API Key Management**: Easily manage API keys for different AI models.
- **Default Model Selection**: Set a default AI model to use for generating commit messages.
- **Interactive Model Selection**: If you're unsure which models are supported, you can run commands without specifying a value to trigger an interactive selection menu.
- **Easy Installation**: Install the tool system-wide with a single command.

## Installation

### Quick Installation

You can install `aimit` system-wide by running the following command:

```bash
wget -qO- https://raw.githubusercontent.com/TutTrue/Aimit/refs/heads/main/install_aimit.sh | sudo bash
```

This script will download and install the `aimit` binary to your system, making it available globally.

### Manual Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-repo/aimit.git
   cd aimit
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Install the binary**:
   ```bash
   cargo install --path .
   ```

## Usage

### Basic Usage

To generate a commit message for the staged changes in your Git repository, simply run:

```bash
aimit
```

This will use the default AI model to generate a commit message.

### Selecting a Different AI Model

You can specify a different AI model using the `-m` or `--model` flag:

```bash
aimit -m gemini
```

If you're unsure which models are supported, you can run the command without specifying a value to trigger an interactive selection menu:

```bash
aimit -m
```

### Setting an API Key

To set the API key for a specific AI model, use the `-k` or `--key` flag:

```bash
aimit -k YOUR_API_KEY -m gemini
```

If you're unsure which models are supported, you can run the command without specifying a value to trigger an interactive selection menu:

```bash
aimit -k YOUR_API_KEY -m
```

### Updating the Default Model

You can update the default AI model using the `-d` or `--default` flag:

```bash
aimit -d gemini
```

If you're unsure which models are supported, you can run the command without specifying a value to trigger an interactive selection menu:

```bash
aimit -d
```

### Customizing the Prompt

To customize the prompt used to generate commit messages, use the `-p` or `--prompt` flag:

```bash
aimit -p
```

This will open your default text editor, allowing you to edit the prompt.

### Updating Aimit

You can update Aimit using the `-u` or `--update` flag:

```bash
aimit -u
```


## Configuration

The tool uses a `Config.toml` file to store settings such as the default AI model, API keys, and the prompt. This file is automatically created the first time you run the tool and is located in the `~/.config/aimit/` directory.

### Example `Config.toml`

```toml
default_model = "GEMINI"
prompt = "Analyze the following git diff and generate a concise and meaningful commit message summarizing the changes.\nThe commit message should follow best practices, including a short title and an optional detailed description if necessary.\ngit diff:\n{}\nRequirements:\n  Title: 50 characters or less, summarizing the change.\n  Optional Description: If the change requires context, provide a brief explanation in the body."

[api_keys]
gemini_api_key = "YOUR_GEMINI_API_KEY"
```

### Config File Location

The configuration file (`Config.toml`) is stored in the following directory:

```
~/.config/aimit/Config.toml
```

You can manually edit this file to update settings such as the default model, API keys, or prompt.

## Supported AI Models

Currently, the tool supports the following AI models:

- **Gemini**: A powerful AI model for generating commit messages.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or new features to suggest.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Enjoy automating your commit messages with `aimit`! 🚀