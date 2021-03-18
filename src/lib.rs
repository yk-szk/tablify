use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
use encoding_rs;
use std::error::Error;
use std::io::{BufReader, Cursor};
use tera::{Context, Tera};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn render(
    template: &str,
    raw_content: &[u8],
    filename: &str,
    has_headers: bool,
    autoescape: bool,
) -> Result<String, JsValue> {
    let html = tablify(template, raw_content, filename, has_headers, autoescape);
    html.map_err(|e| JsValue::from(e.to_string()))
}

/// Load tabular data and render html using the contents and the template
///
/// # Arguments
///
/// * `template`: Template
/// * `raw_content`: Bytes of input tabular data
/// * `filename`: Only used to determine file type (.csv or .xlsx)
/// * `autoescape`: Enable autoescaping in html rendering
pub fn tablify(
    template: &str,
    raw_content: &[u8],
    filename: &str,
    has_headers: bool,
    autoescape: bool,
) -> Result<String, Box<dyn Error>> {
    let table_data = match std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        Some("csv") => Ok(load_csv(&raw_content)?),
        Some("xlsx") => Ok(load_xlsx(&raw_content)?),
        _ => Err("Invalid file extension"),
    };
    render_table(template, table_data?, has_headers, autoescape).map_err(|e| e.into())
}

/// Load CSV file
pub fn load_csv(a: &[u8]) -> Result<Vec<Vec<String>>, csv::Error> {
    let (csv_content, _, _) = encoding_rs::SHIFT_JIS.decode(&a);
    let csv_content = csv_content.into_owned();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_content.as_bytes());
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.deserialize() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

/// Load XLSX file
pub fn load_xlsx(a: &[u8]) -> Result<Vec<Vec<String>>, calamine::Error> {
    let cursor = Cursor::new(a);
    let buf = BufReader::new(cursor);
    let mut workbook = Xlsx::new(buf)?;
    let mut rows: Vec<Vec<String>> = Vec::new();
    let sheet_name = workbook.sheet_names()[0].to_owned();
    let range = workbook
        .worksheet_range(&sheet_name)
        .ok_or(calamine::Error::Msg("Cannot find a sheet."))??;
    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;
    while let Some(result) = iter.next() {
        let row: Vec<String> = result?;
        rows.push(row);
    }
    Ok(rows)
}

/// Render rows using the given template.
pub fn render_table(
    template_content: &str,
    rows: Vec<Vec<String>>,
    has_headers: bool,
    autoescape: bool,
) -> Result<String, tera::Error> {
    let mut context = Context::new();
    if has_headers {
        context.insert("headers", &rows[0]);
        context.insert("rows", &rows[1..]);
    } else {
        context.insert("rows", &rows);
    }
    Tera::one_off(&template_content, &context, autoescape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_csv() {
        let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n");
        let loaded_rows = load_csv(csv_content.as_bytes()).unwrap();
        assert_eq!(rows, loaded_rows);
    }
    #[test]
    fn test_tablify() {
        let template = "{% for row in rows %}{{ row | join(sep=\",\")}}\n{% endfor %}";
        let rows = vec![vec!["r1c1", "r1c2"], vec!["r2c1", "r2c2"]];
        let csv_content = rows
            .iter()
            .map(|row| row.join(","))
            .collect::<Vec<_>>()
            .join("\n");
        let table = tablify(template, csv_content.as_bytes(), "fn.csv", false, false).unwrap();
        println!("{}", table);
        assert_eq!(table, csv_content + "\n");
    }
}
