mod dto;
//use sqlx::Connection;
use calamine::{
    open_workbook, DataType, DeError, Range, RangeDeserializer, RangeDeserializerBuilder, Reader,
    Xls, Xlsx,
};
use serde::{Deserialize, Serialize};
use dto::TestStruct;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).collect::<Vec<_>>();
    let path = Path::new(&filename[0]);
    let mut workbook: Xls<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    //let pg_pool = sqlx::PgPool::connect("postgresql://postgres:postgres@localhost/library").await?;
    let _res = processing_example(path.file_name().unwrap().to_str().unwrap(), &range);
    processing(path.file_name().unwrap().to_str().unwrap(), &range)
}

fn processing_example(
    filename: &str,
    range: &Range<DataType>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut iter: RangeDeserializer<DataType, Vec<String>> = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    if let Some(result) = iter.next() {
        let storage: Vec<String> = result?;
        for item in storage {
            dbg!(item);
        }
    }

    Ok(())
}

fn processing(filename: &str, range: &Range<DataType>) -> Result<(), Box<dyn std::error::Error>> {
    let mut iter: RangeDeserializer<DataType, Vec<String>> = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(range)?;

    match filename {
        "test.xls" => {println!("{:?}", TestStruct::new(iter))},
        _ => {}
    }

    Ok(())
}
