use clap::{Parser};
use dialoguer::{MultiSelect, Input, theme::ColorfulTheme};
use std::process;

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

    let (endpoints, graphql_schemas) = configure_api(selected_api_type);

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

    generate_project(&project_name, selected_api_type, &endpoints, &graphql_schemas);
}

// Function to configure API based on the selected type
fn configure_api(api_type: &str) -> (Vec<Endpoint>, Vec<GraphQLSchema>) {
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
fn generate_project(
    project_name: &str,
    selected_api_type: &str,
    endpoints: &[Endpoint],
    graphql_schemas: &[GraphQLSchema],
) {
    println!(
        "Generating project '{}' with API type: {}",
        project_name, selected_api_type
    );

    if selected_api_type == "REST" {
        println!("Configuring REST API endpoints...");
        for endpoint in endpoints {
            let method_lower = endpoint.method.to_lowercase();
            println!(
                "- Path: {}, Method: {}",
                endpoint.path, method_lower
            );
        }
    } else if selected_api_type == "GraphQL" {
        println!("Configuring GraphQL schemas...");
        for schema in graphql_schemas {
            println!("- Name: {}, Type: {}", schema.name, schema.type_kind);
        }
    }
}
