use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
use encoding_rs;
use std::io::{BufReader, Cursor};
use tera::{Context, Tera};

pub fn tablify(template: &str, raw_content: &Vec<u8>, filename: &str) -> Result<String, String> {
    let table_data = match std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        Some("csv") => Some(load_csv(&raw_content).unwrap()),
        Some("xlsx") => Some(load_xlsx(&raw_content).unwrap()),
        _ => None,
    };
    render_table(template, table_data.unwrap())
}

pub fn load_csv(a: &Vec<u8>) -> Result<Vec<Vec<String>>, csv::Error> {
    let (csv_content, _, _) = encoding_rs::SHIFT_JIS.decode(&a);
    let csv_content = csv_content.into_owned();
    let mut rdr = csv::Reader::from_reader(csv_content.as_bytes());
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

pub fn load_xlsx(a: &Vec<u8>) -> Result<Vec<Vec<String>>, calamine::Error> {
    let cursor = Cursor::new(a);
    let buf = BufReader::new(cursor);
    let mut workbook = Xlsx::new(buf).unwrap();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let sheet_name = workbook.sheet_names()[0].clone(); // TODO: is clone() avoidable?
    let range = workbook
        .worksheet_range(&sheet_name)
        .ok_or(calamine::Error::Msg("Cannot find a sheet."))??;
    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
    while let Some(result) = iter.next() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

pub fn render_table(template_content: &str, rows: Vec<Vec<String>>) -> Result<String, String> {
    let mut context = Context::new();
    context.insert("rows", &rows);
    let html = Tera::one_off(&template_content, &context, true);
    match html {
        Ok(h) => return Ok(h),
        Err(e) => return Err(e.to_string()),
    }
}
