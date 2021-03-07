extern crate clap;
use clap::{crate_version, App, Arg};
use encoding_rs;
use std::fs;
use tera::{Context, Tera};

fn load_table(csv_content: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

fn render_table(template_content: &str, rows: Vec<Vec<String>>) -> Result<String, String> {
    let mut context = Context::new();
    context.insert("rows", &rows);
    let html = Tera::one_off(&template_content, &context, true);
    match html {
        Ok(h) => return Ok(h),
        Err(e) => return Err(e.to_string()),
    }
}

fn main() {
    let matches = App::new("tablify")
        .version(crate_version!())
        .author("Yuki Suzuki <y-suzuki@radiol.med.osaka-u.ac.jp>")
        .about("Load tabular data and render a html file.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input csv file to use")
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
    let csv_content = fs::read(input).unwrap();
    let (csv_content_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&csv_content);
    let csv_content_utf8 = csv_content_utf8.into_owned();
    let rows = load_table(&csv_content_utf8).unwrap();
    let template_content = fs::read_to_string(template_filename).unwrap();

    // let mut context = Context::new();
    // context.insert("rows", &row);
    // let html = Tera::one_off(&template_content, &context, true).unwrap();
    let html = render_table(&template_content, rows).unwrap();
    println!("{}", html);

    std::process::exit(0)
}
