# Dypi

Dypi is a powerful CLI tool for effortlessly creating Rust-based APIs, designed to streamline your API development workflow.

## 🚀 Features

- **Interactive CLI Scaffolding**: Quickly bootstrap your API projects with guided setup
- **Flexible API Types**: 
  - REST API support
  - Powered by the Tide web framework
- **Automatic Project Generation**: 
  - Generates complete project structure
  - Pre-configured boilerplate code
  - Reduces setup time and complexity

## 📦 Installation

Install Dypi globally using Cargo:

```bash
cargo install dypi
```

## 🛠 Usage

Creating a new API project is as simple as running:

```bash
dypi <project_name>
```

### Workflow

1. Run the CLI
2. Select your preferred API type 
3. Follow interactive prompts
4. Navigate to your new project directory
5. Start developing!

## 🌟 Quick Example

```bash
# Create a new API project
dypi my-api

# Navigate to project directory
cd my-api

# Run the API
cargo run
```

Your API will be available at `http://127.0.0.1:8080`

## 🔧 Project Structure

After generating a project, you'll have a typical Rust project structure:

```
my-api/
├── Cargo.toml
├── src/
│   ├── main.rs
└
```

## 🛣️ Roadmap

- [x] REST API
- [ ] GraphQL API 


## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📧 Contact

Published by Yash Yadav, feel free to connect at "yash2yk2@gmail.com"
