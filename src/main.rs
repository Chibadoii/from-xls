use calamine::{
    open_workbook, DataType, DeError, Range, RangeDeserializerBuilder, Reader, Xls, Xlsx,
};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct TestStruct {
    #[serde(default)]
    column1: Option<String>,
    #[serde(default)]
    column2: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).collect::<Vec<_>>();
    let path = Path::new(&filename[0]);
    let mut workbook: Xls<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    processing(path.file_name().unwrap(), range)
}

fn processing(filename: &OsStr, range: Range<DataType>) -> Result<(), Box<dyn std::error::Error>> {
    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    if let Some(result) = iter.next() {
        let storage: Vec<String> = result?;
        for item in storage {
            println!("{:?}", item);
        }
    }

    Ok(())
}
