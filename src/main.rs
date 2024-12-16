mod dto;
use calamine::{open_workbook, DataType, Range, RangeDeserializerBuilder, Reader, Xls};
use dto::AllColumnStruct;
use sqlx::{Connection, Executor, PgConnection};
use std::path::Path;

//temporary
async fn create_table(pg_con: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    let res = pg_con.execute("drop table if exists testing_sk").await?;
    println!("{res:?}");
    let res = pg_con.execute("create table if not exists testing_sk (id integer primary key, field1 text, field2 integer, field3 text)").await?;
    println!("{res:?}");
    // let res = pg_con.execute("insert into testing_sk (id, field1, field2, field3) values (1, 'firsttxt', 1, 'sectxt')").await?;
    // println!("{res:?}");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).collect::<Vec<_>>();
    let path = Path::new(&filename[0]);
    let mut workbook: Xls<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let mut pg_con =
        sqlx::PgConnection::connect("postgres://tester:tester@localhost/testbase").await?;
    let _ = create_table(&mut pg_con).await;

    let from_xls = write_xls(&range)?;
    for row in from_xls {
        one_row_all_table(from_xls, &mut pg_con).await?;
    }
    Ok(())
}

fn write_xls(range: &Range<DataType>) -> Result<Vec<AllColumnStruct>, Box<dyn std::error::Error>> {
    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range(range)?;

    let data: Vec<AllColumnStruct> = iter.map(|item| item.unwrap()).collect();
    println!("{data:?}");
    Ok(data)
}

async fn one_row_all_table(
    from_xls: Vec<AllColumnStruct>,
    pg_con: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    for row in from_xls {
        row.plan_version(pg_con).await
    }
    Ok(())
}
