use clap::{Parser};
use dialoguer::{MultiSelect, Input, Confirm, theme::ColorfulTheme};
use std::fs;
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

fn main() {
    let args = Cli::parse();
    let project_name = args.project_name;

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
                
                let path: String = Input::new()
                    .with_prompt("Enter endpoint path (e.g., /users)")
                    .interact_text()
                    .unwrap();

                let methods = vec!["GET", "POST", "PUT", "DELETE"];
                let method_selection = MultiSelect::new()
                    .with_prompt("Select HTTP methods for this endpoint")
                    .items(&methods)
                    .interact()
                    .unwrap_or_default();

                let selected_methods: Vec<String> = if method_selection.is_empty() {
                    vec!["GET".to_string()]
                } else {
                    method_selection.iter().map(|&idx| methods[idx].to_string()).collect()
                };

                for method in selected_methods {
                    endpoints.push(Endpoint {
                        path: path.clone(),
                        method: method.to_string(),
                    });
                }
            }
        },
        "GraphQL" => {
            let num_schemas: usize = Input::new()
                .with_prompt("How many GraphQL schemas (queries/mutations) do you want to create?")
                .default(2)
                .interact_text()
                .unwrap_or(2);

            for i in 1..=num_schemas {
                println!("\nConfiguring GraphQL Schema #{}", i);
                
                let name: String = Input::new()
                    .with_prompt("Enter schema name (e.g., getUser)")
                    .interact_text()
                    .unwrap();

                let types = vec!["Query", "Mutation"];
                let type_selection = MultiSelect::new()
                    .with_prompt("Select schema type")
                    .items(&types)
                    .interact()
                    .unwrap_or_default();

                let selected_type: String = if type_selection.is_empty() {
                    "Query".to_string()
                } else {
                    types[type_selection[0]].to_string()
                };

                graphql_schemas.push(GraphQLSchema {
                    name: name.clone(),
                    type_kind: selected_type,
                });
            }

            endpoints.push(Endpoint {
                path: "/graphql".to_string(),
                method: "POST".to_string(),
            });
        },
        _ => panic!("Unsupported API type"),
    }

    (endpoints, graphql_schemas)
}

fn generate_project(project_name: &str, api_type: &str, endpoints: &[Endpoint], graphql_schemas: &[GraphQLSchema]) {
    let project_dir = format!("./{}", project_name);
    fs::create_dir_all(&project_dir).expect("Failed to create project directory");

    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
# REST dependencies
tide = "0.16.0"
async-std = "1.12.0"

# GraphQL dependencies
async-graphql = "5.0"
async-graphql-tide = "5.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#,
        name = project_name
    );
    fs::write(format!("{}/Cargo.toml", project_dir), cargo_toml)
        .expect("Failed to write Cargo.toml");

    let src_dir = format!("{}/src", project_dir);
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let main_content = generate_main_rs(api_type, endpoints, graphql_schemas);
    fs::write(format!("{}/main.rs", src_dir), main_content)
        .expect("Failed to write main.rs");

    println!("Project '{}' created successfully!", project_name);
}

fn generate_main_rs(api_type: &str, endpoints: &[Endpoint], graphql_schemas: &[GraphQLSchema]) -> String {
    match api_type {
        "REST" => {
            let routes: String = endpoints.iter().map(|endpoint| {
                format!(
                    r#"
    app.at("{path}").{method}(|_| async {{
        Ok("Endpoint: {path}, Method: {method}")
    }});"#, 
                    path = endpoint.path, 
                    method = endpoint.method.to_lowercase()
                )
            }).collect();

            format!(
                r#"use tide::{{Request, Response, Server}};
use async_std::task;

#[async_std::main]
async fn main() -> tide::Result<()> {{
    let mut app = tide::new();
    
    // Default route
    app.at("/").get(|_| async {{ 
        Ok("Welcome to REST API!") 
    }});

    // Health check endpoint
    app.at("/health").get(|_| async {{
        Ok("API is healthy!")
    }});

    // Generated Endpoints{routes}

    println!("Server starting on http://localhost:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}}
"#
            )
        },
        "GraphQL" => {
            let queries: String = graphql_schemas.iter()
                .filter(|schema| schema.type_kind == "Query")
                .map(|schema| {
                    format!(
                        r#"
    #[Object]
    impl Query {{
        async fn {name}(&self) -> String {{
            "Result of {name} query".to_string()
        }}
    }}"#,
                        name = schema.name
                    )
                }).collect();

            let mutations: String = graphql_schemas.iter()
                .filter(|schema| schema.type_kind == "Mutation")
                .map(|schema| {
                    format!(
                        r#"
    #[Object]
    impl Mutation {{
        async fn {name}(&self) -> String {{
            "Result of {name} mutation".to_string()
        }}
    }}"#,
                        name = schema.name
                    )
                }).collect();

            format!(
                r#"use async_graphql::{{
    Object, Schema, EmptySubscription,
    http::{{GraphQLRequest, GraphQLResponse}},
}};
use async_graphql_tide::GraphQL;
use tide::{{Request, Response}};
use async_std::task;

// Query type
struct Query;

// Mutation type
struct Mutation;

{queries}

{mutations}

#[async_std::main]
async fn main() -> tide::Result<()> {{
    // Create GraphQL schema
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .finish();

    // Create Tide server
    let mut app = tide::new();
    
    // GraphQL endpoint
    app.at("/graphql")
        .post(GraphQL::new(schema.clone()));

    // GraphiQL interface for testing (optional)
    app.at("/graphiql")
        .get(GraphQL::graphiql("/graphql"));

    println!("GraphQL server starting on http://localhost:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}}
"#
            )
        },
        _ => panic!("Unsupported API type"),
    }
}