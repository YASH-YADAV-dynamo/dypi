use clap::{Parser};
use dialoguer::{MultiSelect, Input, theme::ColorfulTheme};
use async_std::task; // Ensure async runtime is used correctly
use std::process;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project_name: String,
}

#[derive(Debug, Clone)]
struct GraphQLSchema {
    name: String,
    type_kind: String,
}

#[derive(Debug, Clone)]
struct Endpoint {
    path: String,
    method: String,
}

// Entry point of the application
fn main() {
    task::block_on(run());
}

// Async function for main logic
async fn run() {
    // Parse CLI arguments
    let args = Cli::parse();
    let project_name = args.project_name;

    // Define available API types
    let api_types = vec!["REST", "GraphQL"];
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of API do you want? (Use SPACE to select, ENTER to confirm)")
        .items(&api_types)
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("No API type selected. Please choose at least one option.");
            process::exit(1);
        });

    if selection.is_empty() {
        eprintln!("No API type selected. Please choose at least one option.");
        process::exit(1);
    }

    let selected_api_type = api_types[selection[0]];

    let (endpoints, graphql_schemas) = configure_api(selected_api_type).await;

    println!("Selected API type: {}", selected_api_type);

    if selected_api_type == "REST" {
        println!("Configured Endpoints:");
        for endpoint in &endpoints {
            println!("- Path: {}, Method: {}", endpoint.path, endpoint.method);
        }
    } else {
        println!("Configured GraphQL Schema:");
        for schema in &graphql_schemas {
            println!("- Name: {}, Type: {}", schema.name, schema.type_kind);
        }
    }

    generate_project(&project_name, selected_api_type, &endpoints, &graphql_schemas).await;
}

// Function to configure API based on the selected type
async fn configure_api(api_type: &str) -> (Vec<Endpoint>, Vec<GraphQLSchema>) {
    let mut endpoints = Vec::new();
    let mut graphql_schemas = Vec::new();

    match api_type {
        "REST" => {
            let num_endpoints: usize = Input::new()
                .with_prompt("How many REST endpoints do you want to create?")
                .default(2)
                .interact_text()
                .unwrap_or(2);

            for i in 1..=num_endpoints {
                println!("\nConfiguring Endpoint #{}", i);

                // Get the path for the endpoint
                let path: String = Input::new()
                    .with_prompt("Enter endpoint path (e.g., /users)")
                    .interact_text()
                    .unwrap();

                // HTTP method selection prompt
                let methods = vec!["GET", "POST", "PUT", "DELETE"];
                let method_selection = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select HTTP methods for this endpoint (Use SPACE to select, ENTER to confirm)")
                    .items(&methods)
                    .interact()
                    .unwrap_or_default();

                // Store each selected method in the `endpoints` vector
                if method_selection.is_empty() {
                    println!("No methods selected. Defaulting to GET.");
                    endpoints.push(Endpoint {
                        path: path.clone(),
                        method: "GET".to_string(),
                    });
                } else {
                    for &idx in method_selection.iter() {
                        endpoints.push(Endpoint {
                            path: path.clone(),
                            method: methods[idx].to_string(),
                        });
                    }
                }
            }
        }
        _ => panic!("Unsupported API type"),
    }

    (endpoints, graphql_schemas)
}

// Function to generate the project output
async fn generate_project(
    project_name: &str,
    selected_api_type: &str,
    endpoints: &[Endpoint],
    graphql_schemas: &[GraphQLSchema],
) {
    println!(
        "Generating project '{}' with API type: {}",
        project_name, selected_api_type
    );

    // Create project directory
    let project_dir = Path::new(project_name);
    if !project_dir.exists() {
        std::fs::create_dir(project_dir).expect("Failed to create project directory");
    }

    // Generate Cargo.toml
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let mut cargo_toml = File::create(&cargo_toml_path).expect("Failed to create Cargo.toml");

    writeln!(
        cargo_toml,
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
tide = "0.16.0"
async-std = {{ version = "1.10", features = ["attributes"] }}
"#,
        name = project_name
    )
    .expect("Failed to write to Cargo.toml");

    // Generate src/main.rs
    let src_dir = project_dir.join("src");
    if !src_dir.exists() {
        std::fs::create_dir(&src_dir).expect("Failed to create src directory");
    }

    let main_rs_path = src_dir.join("main.rs");
    let mut main_rs = File::create(&main_rs_path).expect("Failed to create main.rs");

    writeln!(
        main_rs,
        r#"use tide;

#[async_std::main]
async fn main() -> tide::Result<()> {{
    let mut app = tide::new();
    app.at("/").get(|_| async {{ Ok("Hello, world!") }});
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}}
"#
    )
    .expect("Failed to write to main.rs");

    println!("Project '{}' has been successfully generated!", project_name);
}
