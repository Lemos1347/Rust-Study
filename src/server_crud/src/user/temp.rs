use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeuObjeto {
    pub campo1: String,
    pub campo2: i32,
}
