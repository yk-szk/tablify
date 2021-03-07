extern crate clap;
use clap::{crate_version, App, Arg};
use std::fs;
use std::fs::File;
use tera::{Context, Tera};

fn load_table(file_path: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

fn main() {
    let matches = App::new("tablify")
        .version(crate_version!())
        .author("Yuki Suzuki <y-suzuki@radiol.med.osaka-u.ac.jp>")
        .about("Load tabular data and render a html file.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("TEMPLATE")
                .help("Sets the template file to use")
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let template_filename = matches.value_of("TEMPLATE").unwrap();
    let csv_contents = load_table(input).unwrap();
    let template_content = fs::read_to_string(template_filename).unwrap();

    let mut context = Context::new();
    context.insert("rows", &csv_contents);
    let html = Tera::one_off(&template_content, &context, true).unwrap();
    println!("{}", html);

    std::process::exit(0)
}
