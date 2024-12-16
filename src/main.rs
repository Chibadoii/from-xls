mod dto;
use sqlx::{query, Connection, Executor, PgConnection};
use calamine::{
    open_workbook, DataType, DeError, Range, RangeDeserializer, RangeDeserializerBuilder, Reader,
    Xls, Xlsx,
};
use serde::{Deserialize, Serialize};
use dto::AllColumnStruct;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).collect::<Vec<_>>();
    let path = Path::new(&filename[0]);
    let mut workbook: Xls<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let mut pg_con = sqlx::PgConnection::connect("postgres://tester:tester@localhost/testbase").await?;
    let _ = create_table(&mut pg_con).await;

    let value = write_xls(path.file_name().unwrap().to_str().unwrap(), &range)?;
    value.into_iter().for_each(|row| {});

    Ok(())
}

fn write_xls(filename: &str, range: &Range<DataType>) -> Result<Vec<AllColumnStruct>, Box<dyn std::error::Error>> {
    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(range)?;

    let data: Vec<AllColumnStruct> = iter.map(|item| item.unwrap()).collect();
    println!("{data:?}");
    Ok(data)
}

async fn create_table(pg_con: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    let res = pg_con.execute("drop table if exists testing_sk").await?;
    println!("{res:?}");
    let res = pg_con.execute("create table if not exists testing_sk (id integer primary key, field1 text, field2 integer, field3 text)").await?;
    println!("{res:?}");
    let res = pg_con.execute("insert into testing_sk (id, field1, field2, field3) values (1, 'firsttxt', 1, 'sectxt')").await?;
    println!("{res:?}");
    Ok(())
}
