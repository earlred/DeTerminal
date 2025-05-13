# DeTerminal: Semantic Terminal AI Assistant

DeTerminal is an AI-assisted terminal tool that suggests and corrects shell commands in real-time using OpenAI or a locally running Ollama model. It helps you write correct shell commands and provides explanations for your actions.

## 🚀 Features

* 🤖 AI-powered command suggestions and corrections
* 🔄 Real-time command validation
* 🌐 Multiple AI backends (OpenAI GPT-4 or local Ollama models)
* 🔒 Privacy-focused with local model support
* 💻 Cross-platform: tested on macOS and Linux
* 🎯 Natural language understanding
* 🛠️ Runs entirely in your terminal

## 🛠️ Installation


#### Prerequisites

- Rust (install from [https://rustup.rs](https://rustup.rs))
- For OpenAI: an OpenAI API key
- For Ollama: Ollama installed locally
- For macOS DMG packaging: [Homebrew](https://brew.sh/) and `create-dmg` (`brew install create-dmg`)

#### Build and Install

```bash
git clone https://github.com/earlred/determinal.git
cd determinal
cargo install --path .
```

#### Build macOS DMG (optional, macOS only)

If you want to package a DMG installer on macOS:

```bash
brew install create-dmg
cargo build --release
# The workflow will output DMG to output/DeTerminal.dmg
```

#### Build Windows EXE Installer (optional, Windows only)

The workflow will output the EXE installer to `installer/windows/Output/DeTerminal-Setup.exe`.

## 🔐 AI Backend Setup

DeTerminal supports two AI backends:

### OpenAI (GPT-4)

1. Get your API key from [OpenAI API Keys page](https://platform.openai.com/account/api-keys)
2. Set your key as an environment variable:

<details>
<summary>macOS/Linux (bash)</summary>

```bash
echo 'export OPENAI_API_KEY=your-api-key-here' >> ~/.bashrc
source ~/.bashrc
```

</details>

<details>
<summary>macOS/Linux (zsh)</summary>

```zsh
echo 'export OPENAI_API_KEY=your-api-key-here' >> ~/.zshrc
source ~/.zshrc
```

</details>

<details>
<summary>Windows (PowerShell)</summary>

```powershell
$env:OPENAI_API_KEY="your-api-key-here"
```

</details>

### Ollama (Local)

Ollama is an open-source framework for running LLMs locally. Perfect for offline or privacy-focused use.

#### Installation

<details>
<summary>macOS</summary>

```bash
# Using Homebrew
brew install ollama

# Or download the .dmg from ollama.ai
```

</details>

<details>
<summary>Linux</summary>

```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

</details>

<details>
<summary>Windows</summary>

Download the installer from [ollama.ai](https://ollama.ai)

</details>

#### Running Ollama

1. Start the Ollama service:
   ```bash
   ollama serve
   ```

2. Pull the recommended model:
   ```bash
   ollama pull gemma3:4b
   ```

3. Keep the Ollama service running in a separate terminal window while using DeTerminal.

> **Note:** The `gemma3:4b` model has been tested to work best with DeTerminal. Compatibility with other models is still being improved.

## 🧪 Usage

Start DeTerminal by running:
```bash
determinal
```

When you first start DeTerminal, it will prompt you to choose your preferred AI source. You can change the AI provider at any time by typing `change-ai`.

### Example Commands

```shell
# List files in current directory
DeTerminal > list files
💭 Validating with AI...
🤖 I think you meant to run: `ls`
✔ ❓ Do you want to run it? · yes

# Switch AI provider
DeTerminal > change-ai
Select your AI provider:
  [0] OpenAI (GPT-4)
  [1] Ollama (local)
> 0

# Get help
DeTerminal > help
💭 Validating with AI...
🤖 I think you meant to run: `man bash`
ℹ️  The "help" command is not recognized in MacOS terminal. To get help about a specific command, use "man" followed by the command name.
✔ ❓ Do you want to run it? · yes
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

MIT

---

Made with ❤️ by [Earl Red](https://github.com/earlred)
