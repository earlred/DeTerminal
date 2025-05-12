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

To use the OpenAI backend, set your API key by adding the following line to your shell config file:

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

> Replace `your-api-key-here` with your actual OpenAI key. You can get one from [https://platform.openai.com/account/api-keys](https://platform.openai.com/account/api-keys)

## 🤖 AI Backends

You can choose between:

* **OpenAI (GPT-4)** — requires setting `OPENAI_API_KEY`
* **Ollama (local)** — runs a local model like llama3: `ollama run llama3`

When DeTerminal starts, it will prompt you to choose your preferred AI source.

## 🧪 Example Usage

```shell
FixBot > list files
💭 Validating with AI...
🤖 I think you meant to run: `ls`
✔ ❓ Do you want to run it? · yes
```

## 📝 License

MIT

---

Made with ❤️ by [Earl Red](https://github.com/earled)
