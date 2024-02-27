use std::collections::HashMap;
use std::fmt::Error;
use axum::http::StatusCode;
use axum::Json;

use serde::{Deserialize, Serialize};

pub async fn root() -> &'static str {
    "matching!"
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Data{
    candidate: Vec<Candidate>,
    attributes: HashMap<String,f64>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Response<T> {
    code: u32,
    message: String,
    data: T,
}


pub async fn matching(
    Json(payload): Json<Data>,
) -> (StatusCode, Json<Response<Vec<Candidate>>>) {
    return match set_score(payload.candidate, payload.attributes).await {
        Ok(candidates) => {
            (StatusCode::OK, Json(Response {
                code: 200,
                message: "OK".to_string(),
                data: candidates,
            }))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                code: 500,
                message: format!("service error:{}", e),
                data: vec![],
            }))
        }
    }
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Candidate {
    name: Option<String>,
    age: Option<String>,
    gender: Option<String>,
    address: Option<String>,
    score: f64,
}

//传入所有候选人，按照打分进行排序，可以采用最大堆或者快排
//可以通过多线程优化计算
async fn set_score(mut candidates: Vec<Candidate>, attributes: HashMap<String,f64>) -> Result<Vec<Candidate>,Error>{
    println!("{:?}",attributes);
    let score = 0.0;
    candidates.sort_by_key(|k| k.score);
    candidates.reverse();
    Ok(candidates)
}


