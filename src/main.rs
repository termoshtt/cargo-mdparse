use std::fs::File;
use std::io::{Read, Write};
use std::path::*;
use std::process::Command;

use clap::{App, Arg, SubCommand};
use pulldown_cmark::*;
use serde::Deserialize;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Deserialize)]
struct LocateProject {
    root: String,
}

fn get_output_dir() -> Result<PathBuf> {
    let cmd = Command::new("cargo").arg("locate-project").output()?;
    let output = String::from_utf8(cmd.stdout)?;
    let lp: LocateProject = serde_json::from_str(&output)?;
    let cargo_toml = Path::new(&lp.root);
    let root = cargo_toml.parent().unwrap();
    let examples = root.join("examples");
    ::std::fs::create_dir_all(&examples)?;
    Ok(examples)
}

struct Args {
    path: PathBuf,
    save_anonymous_block: bool,
}

fn parse_args() -> Args {
    let matches = App::new("cargo-mdparse")
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("mdparse")
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
                ),
        )
        .get_matches();
    let matches = matches.subcommand_matches("mdparse").unwrap();
    let path = matches.value_of("markdown_path").unwrap();
    let save_anonymous_block = matches.is_present("anonymous");
    Args {
        path: PathBuf::from(path),
        save_anonymous_block,
    }
}

fn main() {
    let args = parse_args();
    let output = get_output_dir().expect("Faild to create examples/");

    let mut f = File::open(args.path).unwrap();
    let mut md = String::new();
    f.read_to_string(&mut md).unwrap();
    let parser = Parser::new(&md);

    let mut filename: Option<String> = None;
    let mut current = Vec::new();
    let mut anonymous_count = 0;
    for event in parser {
        match event {
            Event::Start(ty) => match ty {
                Tag::CodeBlock(langtype) => {
                    let langtype = match langtype {
                        CodeBlockKind::Fenced(langtype) => langtype,
                        _ => continue,
                    };
                    if !langtype.starts_with("rust") {
                        continue;
                    }
                    let sp: Vec<_> = langtype.split(':').collect();
                    filename = if sp.len() == 2 {
                        let filename = sp.last().unwrap();
                        if filename.ends_with(".rs") {
                            Some(filename.to_string())
                        } else {
                            eprintln!("Rust snippet is named without `.rs`. Skip it");
                            None
                        }
                    } else {
                        if args.save_anonymous_block {
                            anonymous_count += 1;
                            Some(format!("mdparse{}.rs", anonymous_count))
                        } else {
                            None
                        }
                    };
                }
                _ => {}
            },
            Event::End(_) => {
                if let Some(ref filename) = filename {
                    let contents = current.join("");
                    current.clear();
                    write_to_file(&output, filename, &contents).expect("Failed to save examples");
                }
                filename = None;
            }
            Event::Text(text) => {
                if filename.is_some() {
                    current.push(text);
                }
            }
            _ => {}
        }
    }
}

fn write_to_file(output: &Path, filename: &str, contents: &str) -> Result<()> {
    let filename = output.join(filename);
    let mut f = File::create(filename)?;
    f.write_all(&contents.as_bytes())?;
    Ok(())
}
