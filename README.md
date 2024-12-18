Dypi
Dypi is a CLI tool to create Rust-based APIs with ease. It scaffolds fully functional API projects using the Tide web framework, empowering developers to focus on their application's logic rather than setup boilerplate.

Whether you're building RESTful APIs or GraphQL endpoints, Dypi is your starting point to quickly bootstrap your API projects.

Features
Interactive CLI to scaffold APIs.
Choose between different API types (e.g., REST, GraphQL).
Automatically generates project structure and files.
Pre-configured with the Tide web framework.
Installation
To use Dypi, first install the crate globally using cargo:

bash:
Copy code
cargo install dypi
Usage
Run the CLI:

bash
Copy code
dypi <project_name>
Follow the prompts to select the type of API you want to create:

REST API
GraphQL API (future implementation)
Navigate to the generated project directory and build your API:

bash
Copy code
cd <project_name>
cargo run
Example
Here’s a quick example to create a new REST API project:

bash
Copy code
# Create a new project named "my-api"
dypi my-api

# Navigate into the project directory
cd my-api

# Run the API
cargo run
The API will be available at http://127.0.0.1:8080.