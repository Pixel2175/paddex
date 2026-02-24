mod utils;

use clap::Parser;
use minijinja::{Environment, Value};
use std::io::{self, IsTerminal, Read};
use toml::Value as TomlValue;
use crate::utils::*;

#[derive(Parser,Debug)]
struct Args {
    #[arg(short, long)]
    layout: Option<String>,

    #[arg(short, long)]
    template: Option<String>,

    #[arg(short,long)]
    config: String,
}

fn main() {
    let args = Args::parse();
    let toml_value:TomlValue = match toml::from_str(&read_file(&args.config)) {
        Ok(value) => value,
        Err(e) =>  die("config", &e.to_string()),
    };
    let context = match toml_value.get("context") {
        Some(value) => Value::from_serialize(&serde_json::to_value(value).unwrap_or_default()),
        None => die("context", "not defined in config.toml"),
    };
    let layout_dir = args.layout.unwrap_or_default();

    let mut template_content = String::new(); 
    let mut template_name    = String::from("stdin");

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut template_content).unwrap();
    } else {
        match args.template {
            Some(v) => {
                template_name    = basename(&v);
                template_content = read_file(&v);
            },
            None    =>{ die("template", "missing template path.")},
        }
    }

    let mut env = Environment::new();
    let layout_file = if !layout_dir.is_empty() { read_file(&layout_dir) } else { String::new() };
    let layout_name = if !layout_dir.is_empty() { basename(&layout_dir) } else { String::new() };

    if !layout_dir.is_empty() {
        if let Err(e) = env.add_template(&layout_name, &layout_file) {
            die("generate", &format!("{e:#}"));
        }
    };

    if let Err(e) = env.add_template(&template_name, &template_content) {
        die("generate", &format!("{e:#}"));
    }

    match env.get_template(&template_name) {
        Ok(v) => match v.render(context)
            {
                Ok(r)  => println!("{}",r),
                Err(e) => die("render", &format!("{e:#}"))
            },
        Err(e) => die("generate", &e.to_string()),
    }
}
