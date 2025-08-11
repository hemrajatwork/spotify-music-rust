use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIResponse<T>{
    pub msg: T,
    pub code: u32
}

