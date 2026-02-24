mod utils;

use std::io::{self, IsTerminal, Read};
use clap::Parser;
use crate::utils::{die,read_file};

#[derive(Parser,Debug)]
struct Args {

    #[arg(short, long)]
    md_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut md_content = String::new(); 

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut md_content).unwrap();
    } else {
        match args.md_file {
            Some(v) =>  md_content = read_file(&v),
            None    =>{ die("markdown", "missing markdown path.")},
        }
    }

    let filters = vec![
    // filters to escape Jinja keywords
           ("<p>{%"        , "{%"  ),
           ("<p>{{"        , "{{"  ),
           ("%}</p>"       , "%}"  ),
           ("}}</p>"       , "}}"  ),
    ];

    let parser = pulldown_cmark::Parser::new(&md_content);
    let mut html = String::new();

    pulldown_cmark::html::push_html(&mut html, parser);

    let mut html_output = String::new();
    for line in html.lines() {
        let mut filtered_line = String::from(line);
        for filter in &filters {
            if line.contains(filter.0) {
                filtered_line = filtered_line.replace(filter.0,filter.1);
            }
        }
        html_output.push_str(&filtered_line);
        html_output.push('\n');
    }

    print!("{}",html_output);
}
