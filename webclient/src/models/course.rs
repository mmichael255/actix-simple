use chrono::NaiveDateTime;
use serde::{Deserialize, Serilize};
use wasm_binden::JsCast;
use wasm_binden_futures::Jsfuture;
use web_sys::{ReqeustInit, Request, RequestMode, Response};

#[derive(Serialize, Debug, Serialize)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
