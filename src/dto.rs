use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{Executor, PgConnection};

#[derive(Debug, Serialize, Deserialize)]
pub struct AllColumnStruct {
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub field1: Option<String>,
    #[serde(default)]
    pub field2: Option<i32>,
    #[serde(default)]
    pub field3: Option<String>,
}
impl AllColumnStruct {
    pub async fn plan_version(&self, pg_con: &mut PgConnection) {
        let res = pg_con
            .execute(
                sqlx::query("insert into testing_sk (id, field1) values ($1, $2)")
                    .bind(&self.id)
                    .bind(&self.field1),
            )
            .await
            .unwrap();
        check_operation(res);
    }
}

fn check_operation(res_query_ops: PgQueryResult) {
    if res_query_ops.rows_affected() == 0 {
        eprintln!("Error pg operation")
    }
}
