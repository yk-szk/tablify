extern crate clap;
use clap::{crate_version, App, Arg};
use std::error::Error;
use std::fs::File;
// use std::io;
use tera::Tera;


fn example(file_path: &str) -> Result<(Vec<csv::StringRecord>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut v: Vec<csv::StringRecord> = Vec::new();
    for result in rdr.records() {
        if let Ok(r) = result {
            println!("{:?}", r);
            v.push(r);
        }
    }
    Ok(v)
}

fn main() {
    let matches = App::new("tablify")
        .version(crate_version!())
        .author("Yuki Suzuki <y-suzuki@radiol.med.osaka-u.ac.jp>")
        .about("Load tabular data and render a html file.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input);

    // let tera = match Tera::new("../table.html") {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("Parsing error(s): {}", e);
    //         ::std::process::exit(1);
    //     }
    // };

    if let Err(err) = example(input) {
        println!("error running example: {}", err);
        std::process::exit(1);
    }

    std::process::exit(0)
}
