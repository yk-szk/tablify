extern crate simple_excel_writer;

use excel::*;
use simple_excel_writer as excel;

use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use tempfile::Builder;

fn create_xlsx(filename: &str, rows: &[Vec<&str>]) {
    let mut wb = Workbook::create(filename);
    let mut sheet = wb.create_sheet("TestSheet");

    wb.write_sheet(&mut sheet, |sw: &mut SheetWriter| {
        rows.iter().try_for_each(|r| {
            let mut row = excel::Row::new();
            for e in r {
                row.add_cell(e.to_owned());
            }
            sw.append_row(row)
        })
    })
    .expect("write excel error!");

    wb.close().expect("close excel error!");
}

#[test]
fn test_load_xlsx() -> Result<()> {
    let tempfile = Builder::new().suffix(".xlsx").tempfile()?;
    let filename = tempfile
        .path()
        .to_str()
        .ok_or(anyhow::anyhow!("Failed to convert path to string"))?;

    let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
    create_xlsx(filename, &rows);
    let mut reader = BufReader::new(File::open(filename)?);
    let metadata = std::fs::metadata(filename)?;
    println!("{}", metadata.len());
    let mut buffer = vec![0; metadata.len() as usize];
    reader.read_exact(&mut buffer)?;
    let loaded_rows = tablify::load_xlsx(&buffer)?;
    assert_eq!(rows, loaded_rows[0]);
    Ok(())
}

#[test]
fn test_tablify() -> Result<()> {
    const TEMPLATE: &str = "{% for row in rows %}{{ row | join(sep=\",\")}}\n{% endfor %}";
    // test csv
    {
        let tempfile = Builder::new().suffix(".csv").tempfile()?;
        let filename = tempfile
            .path()
            .to_str()
            .ok_or(anyhow::anyhow!("Failed to convert path to string"))?;
        let rows = [vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n");
        let table = tablify::tablify(TEMPLATE, csv_content.as_bytes(), filename, false, false)?;
        println!("{}", table);
        assert_eq!(table, csv_content + "\n");
    }

    // test xlsx
    {
        let tempfile = Builder::new().suffix(".xlsx").tempfile()?;
        let filename = tempfile
            .path()
            .to_str()
            .ok_or(anyhow::anyhow!("Failed to convert path to string"))?;
        let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        create_xlsx(filename, &rows);
        let mut reader = BufReader::new(File::open(filename)?);
        let metadata = std::fs::metadata(filename)?;
        let mut buffer = vec![0; metadata.len() as usize];
        reader.read_exact(&mut buffer)?;
        let html = tablify::tablify(TEMPLATE, &buffer, filename, false, false)?;
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        println!("{}", html);
        assert_eq!(csv_content, html);
    }
    Ok(())
}
