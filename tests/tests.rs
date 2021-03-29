extern crate simple_excel_writer;

use excel::*;
use simple_excel_writer as excel;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use tempfile::Builder;

fn create_xlsx(filename: &str, rows: &Vec<Vec<&str>>) {
    let mut wb = Workbook::create(filename);
    let mut sheet = wb.create_sheet("TestSheet");

    wb.write_sheet(&mut sheet, |sw: &mut SheetWriter| {
        rows.iter()
            .map(|r| {
                let mut row = excel::Row::new();
                for e in r {
                    row.add_cell(e.to_owned());
                }
                sw.append_row(row)
            })
            .collect()
    })
    .expect("write excel error!");

    wb.close().expect("close excel error!");
}

#[test]
fn test_load_xlsx() {
    let tempfile = Builder::new().suffix(".xlsx").tempfile().unwrap();
    let filename = tempfile.path().to_str().unwrap();
    let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
    create_xlsx(filename, &rows);
    let mut reader = BufReader::new(File::open(filename).unwrap());
    let metadata = std::fs::metadata(&filename).unwrap();
    println!("{}", metadata.len());
    let mut buffer = vec![0; metadata.len() as usize];
    reader.read(&mut buffer).unwrap();
    let loaded_rows = tablify::load_xlsx(&buffer).unwrap();
    assert_eq!(rows, loaded_rows);
}

#[test]
fn test_tablify() {
    const TEMPLATE: &str = "{% for row in rows %}{{ row | join(sep=\",\")}}\n{% endfor %}";
    // test csv
    {
        let tempfile = Builder::new().suffix(".csv").tempfile().unwrap();
        let filename = tempfile.path().to_str().unwrap();
        let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n");
        let table =
            tablify::tablify(TEMPLATE, csv_content.as_bytes(), filename, false, false).unwrap();
        println!("{}", table);
        assert_eq!(table, csv_content + "\n");
    }

    // test xlsx
    {
        let tempfile = Builder::new().suffix(".xlsx").tempfile().unwrap();
        let filename = tempfile.path().to_str().unwrap();
        let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        create_xlsx(filename, &rows);
        let mut reader = BufReader::new(File::open(filename).unwrap());
        let metadata = std::fs::metadata(&filename).unwrap();
        let mut buffer = vec![0; metadata.len() as usize];
        reader.read(&mut buffer).unwrap();
        let html = tablify::tablify(TEMPLATE, &buffer, &filename, false, false).unwrap();
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        println!("{}", html);
        assert_eq!(csv_content, html);
    }
}
