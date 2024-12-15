mod dto;
use calamine::{
    open_workbook, DataType, DeError, Range, RangeDeserializer, RangeDeserializerBuilder, Reader,
    Xls, Xlsx,
};
use dto::TestStruct;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).collect::<Vec<_>>();
    let path = Path::new(&filename[0]);
    let mut workbook: Xls<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    processing(path.file_name().unwrap().to_str().unwrap(), range).await?;
    Ok(())
}

async fn processing(filename: &str, range: Range<DataType>) -> Result<(), Box<dyn std::error::Error>> {
    let iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;
    let values: Vec<TestStruct> = iter.map(|item| item.unwrap()).collect();
    println!("values: {:?}", values);

   //TODO вытаскиваем определенные поля  и закидываем бд
}
