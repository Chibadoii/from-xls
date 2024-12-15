use calamine::{DataType, RangeDeserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestStruct {
    #[serde(default)]
    pub fields1: Option<String>,
    #[serde(default)]
    pub fields2: Option<String>,
}

// impl TestStruct {
//     fn new(iter: RangeDeserializer<DataType, >) -> Self {
//         todo!()
//     }
// }
