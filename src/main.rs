use std::{process::Command, string};

use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "lm")]
#[command(about = "A mirror setting cli for lazy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Set {
        name: String,
    },
    Unset {
        name: String,
    },
}

fn all(action: i32) {
    todo!()
}

fn node(name: &String, commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::node::set(name);
        }
        Commands::Unset { name: _ } => {
            lm::node::unset(name);
        }
    };
}

fn go(commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::go::set();
        }
        Commands::Unset { name: _ } => {
            lm::go::unset();
        }
    };
}

fn python(name: &String, commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::python::set(name);
        }
        Commands::Unset { name: _ } => {
            lm::python::unset(name);
        }
    };
}

fn php(commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::php::set();
        }
        Commands::Unset { name: _ } => {
            lm::php::unset();
        }
    };
}

fn ruby(commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::ruby::set();
        }
        Commands::Unset { name: _ } => {
            lm::ruby::unset();
        }
    };
}

fn maven(action: i32) {
    todo!()
}

fn gradle(action: i32) {
    todo!()
}

fn rust(commands: &Commands) {
    match commands {
        Commands::Set { name: _ } => {
            lm::rust::set();
        }
        Commands::Unset { name: _ } => {
            lm::rust::unset();
        }
    };
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Set { name } => match name.as_str() {
            "all" => all(1),
            // node
            "npm" => node(name, &args.command),
            "pnpm" => node(name, &args.command),
            "yarn" => node(name, &args.command),
            "node" => node(&String::from("npm"), &args.command),
            // go
            "go" => go(&args.command),
            // python
            "pip" => python(name, &args.command),
            "pip3" => python(name, &args.command),
            "python" => python(&String::from("pip3"), &args.command),
            // php
            "composer" => php(&args.command),
            "php" => php(&args.command),
            // ruby
            "gem" => ruby(&args.command),
            "ruby" => ruby(&args.command),
            "gems" => ruby(&args.command),
            "rubygems" => ruby(&args.command),
            // java
            "maven" => maven(1),
            "gradle" => gradle(1),
            "java" => maven(1),
            // rust
            "cargo" => rust(&args.command),
            "rust" => rust(&args.command),
            // other
            _ => println!("not support it"),
        },
        Commands::Unset { name } => match name.as_str() {
            "all" => all(2),
            // node
            "npm" => node(name, &args.command),
            "pnpm" => node(name, &args.command),
            "yarn" => node(name, &args.command),
            "node" => node(&String::from("npm"), &args.command),
            // go
            "go" => go(&args.command),
            // python
            "pip" => python(name, &args.command),
            "pip3" => python(name, &args.command),
            "python" => python(&String::from("pip3"), &args.command),
            // php
            "composer" => php(&args.command),
            "php" => php(&args.command),
            // ruby
            "gem" => ruby(&args.command),
            "ruby" => ruby(&args.command),
            "gems" => ruby(&args.command),
            "rubygems" => ruby(&args.command),
            // java
            "maven" => maven(2),
            "gradle" => gradle(2),
            "java" => maven(2),
            // rust
            "cargo" => rust(&args.command),
            "rust" => rust(&args.command),
            // other
            _ => println!("not support it"),
        },
    }
}
