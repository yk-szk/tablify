extern crate clap;
use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
use clap::{crate_version, App, Arg};
use encoding_rs;
use std::fs;
use std::io::BufReader;
use tera::{Context, Tera};

fn load_csv(csv_content: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

fn load_xlsx(a: &Vec<u8>) -> Result<Vec<Vec<String>>, calamine::Error> {
    let cursor = std::io::Cursor::new(a);
    let buf = BufReader::new(cursor);
    let mut workbook = Xlsx::new(buf).unwrap();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let sheet_name = workbook.sheet_names()[0].clone(); // TODO: is clone() avoidable?
    let range = workbook
        .worksheet_range(&sheet_name)
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;
    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
    while let Some(result) = iter.next() {
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

    let input = std::path::Path::new(matches.value_of("INPUT").unwrap());
    let raw_content = fs::read(input).unwrap();
    let template_filename = matches.value_of("TEMPLATE").unwrap();
    let template_content = fs::read_to_string(template_filename).unwrap();
    let table_data = match input.extension().and_then(std::ffi::OsStr::to_str) {
        Some("csv") => {
            let (csv_content_utf8, _, _) = encoding_rs::SHIFT_JIS.decode(&raw_content);
            let csv_content_utf8 = csv_content_utf8.into_owned();
            Some(load_csv(&csv_content_utf8).unwrap())
        }
        Some("xlsx") => Some(load_xlsx(&raw_content).unwrap()),
        _ => None,
    };

    let html = render_table(&template_content, table_data.unwrap()).unwrap();
    println!("{}", html);

    std::process::exit(0)
}
