
extern crate pulldown_cmark;
extern crate clap;

use std::fs::File;
use std::io::Read;

use pulldown_cmark::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("cargo-mdparse")
        .arg(
            Arg::with_name("markdown_path")
                .required(true)
                .takes_value(true)
                .help("Path to Markdown file"),
        )
        .arg(
            Arg::with_name("anonymous")
                .help("Parse anonymous code block")
                .short("a")
                .long("anonymous"),
        )
        .get_matches();
    let path = matches.value_of("markdown_path").unwrap();

    let mut f = File::open(path).unwrap();
    let mut md = String::new();
    f.read_to_string(&mut md).unwrap();

    let parser = Parser::new(&md);

    let mut in_code_block = false;
    for com in parser {
        match com {
            Event::Start(ty) => {
                match ty {
                    Tag::CodeBlock(langtype) => {
                        if langtype.starts_with("rust:") {
                            println!("=== start () ===");
                            in_code_block = true;
                        }
                    }
                    _ => {}
                }
            }
            Event::End(ty) => {
                match ty {
                    Tag::CodeBlock(langtype) => {
                        if langtype.starts_with("rust:") {
                            println!("=== end ===");
                            in_code_block = false;;
                        }
                    }
                    _ => {}

                }
            }
            Event::Text(text) => {
                if in_code_block {
                    print!("{}", text);
                }
            }
            _ => {}
        }
    }
}
