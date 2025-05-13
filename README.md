# DeTerminal: Semantic Terminal AI Assistant

DeTerminal is an AI-assisted terminal tool that suggests and corrects shell commands in real-time using OpenAI or a locally running Ollama model.

## 🚀 Features

* Understands natural language queries like "count files" or "fix this command"
* Suggests and auto-corrects mistyped or invalid shell commands
* Works with either OpenAI API or a local Ollama model (e.g. llama3)
* Cross-platform: tested on macOS and Linux
* Runs entirely in your terminal

## 🛠️ Installation

### Prerequisites

* Rust (install from [https://rustup.rs](https://rustup.rs))

### Build and install

```bash
cargo install --path .
```

Once installed, you can run it globally via:

```bash
determinal
```

## 🔐 Environment Variables

To use the OpenAI backend, you need an OpenAI API key. Here's how to get one:

### How to get an OpenAI API key
1. Go to the [OpenAI API Keys page](https://platform.openai.com/account/api-keys) (you'll need to log in or create an account).
2. Click the "Create new secret key" button.
3. Copy the generated key and keep it somewhere safe (you won't be able to see it again).
4. Set your API key as an environment variable:

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

> Replace `your-api-key-here` with your actual OpenAI key.

## 🦙 Ollama Setup

Ollama is an open-source framework for running large language models locally. It's a great alternative to OpenAI when you want to keep your data private or work offline.

### Installing Ollama

1. Visit [Ollama's official website](https://ollama.ai) and download the installer for your operating system.
2. Follow the installation instructions for your platform:

<details>
<summary>macOS</summary>

```bash
# Using Homebrew
brew install ollama

# Or download the .dmg file from the website
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

Download and run the Windows installer from the [Ollama website](https://ollama.ai).

</details>

### Running Ollama

1. Start the Ollama service:
   ```bash
   ollama serve
   ```

2. Pull the recommended model:
   ```bash
   ollama pull gemma3:4b
   ```

3. Keep the Ollama service running in a separate terminal window while using DeTerminal.

## 🤖 AI Backends

You can choose between:

* **OpenAI (GPT-4)** — requires setting `OPENAI_API_KEY`
* **Ollama (local)** — runs a local model like llama3: `ollama run llama3`

> **Note:** When using Ollama, the `gemma3:4b` model has been tested to work best. Compatibility with other models is still being improved.

When DeTerminal starts, it will prompt you to choose your preferred AI source.

## 🧪 Example Usage

```shell
DeTerminal > list files
💭 Validating with AI...
🤖 I think you meant to run: `ls`
✔ ❓ Do you want to run it? · yes

DeTerminal > change-ai
Which AI should FixBot use?
  [0] OpenAI (GPT-4)
  [1] Ollama (local)
> 0

DeTerminal > help
💭 Validating with AI...
🤖 I think you meant to run: `man bash`
ℹ️  The "help" command is not recognized in MacOS terminal. To get help about a specific command, use "man" followed by the command name.
✔ ❓ Do you want to run it? · yes
```

## 📝 License

MIT

---

Made with ❤️ by [Earl Red](https://github.com/earled)
