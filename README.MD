# ⚙️ Nephelios CLI

**Nephelios CLI** is a command-line tool designed to simplify the deployment of applications to the **Nephelios PaaS** platform. It allows developers to deploy, manage, and monitor cloud applications directly from public GitHub repositories — all from their terminal.

---

## 🛠️ Installation

### 🔧 Prerequisites

Make sure the following tools are installed on your system:

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)

### 📦 Installation Steps

1. **Clone the repository**

   ```bash
   git clone https://github.com/Nephelios/nephelios-cli.git

   cd nephelios-cli
   ```

2. **Build the CLI locally**

   ```bash
   cargo build
   ```

   This will generate a local executable in the `target/debug` folder.

3. **(Optional) Global installation**

   To make the `nephelios-cli` command available globally on your system:

   ```bash
   cargo install --path .
   ```

---

## 🚀 Key Features

- ✅ **Easy deployments** from public GitHub repositories.
- ✅ **Supports multiple app types**: Node.js, Python, and more.
- ✅ **Interactive CLI interface** with loading animations and progress indicators.
- ✅ **Robust error handling** and clear feedback for every operation.
- ✅ **Application lifecycle management**: deploy, stop, delete.

---

## 📖 Usage Examples

### 🔼 Deploy Nephelios

```bash
nephelios-cli up
```

This command will:

- Start the Nephelios services
- Deploy the Nephelios stack

### 🔽 Stop and remove a deployed application

```bash
nephelios-cli down
```

This will:

- Stop the running container
- Remove the stack associated with the application
- Clean up resources in the Nephelios environment

---

## 📌 Available Commands

You can explore all available commands using:

```bash
nephelios-cli --help
```

---

## 🤝 Contributing

Contributions are welcome! Whether it's a bug fix, feature request, or documentation improvement:

1. Fork the repository
2. Create a feature branch
3. Open a pull request

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

---

## 📄 License
