mod scoring_rules;

use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use std::fmt::Error;

use serde::{Deserialize, Serialize};

pub async fn root() -> &'static str {
    "matching!"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    candidate: Candidate,             //当前候选匹配条件
    candidates: Vec<Candidate>,       //候选人
    attributes: HashMap<String, f64>, //属性权重
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    code: u32,
    message: String,
    data: T,
}

pub async fn matching(Json(payload): Json<Data>) -> (StatusCode, Json<Response<Vec<Candidate>>>) {
    return match set_score(payload.candidates, payload.candidate, payload.attributes).await {
        Ok(candidates) => (
            StatusCode::OK,
            Json(Response {
                code: 200,
                message: "OK".to_string(),
                data: candidates,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                code: 500,
                message: format!("service error:{}", e),
                data: vec![],
            }),
        ),
    };
}

///打分策略
/// birth_year 通过年龄匹配，正好合适就给出所有权重分数，
/// work传入一个工作栈，出栈即为上一层工作类型，每上一层惩罚分数就会抛物线型加大
/// qualification 学历如果高于匹配学历进行小惩罚，低于匹配学历进行大惩罚
/// current_place传入一个地区栈，出栈顺序区，市，省。逐层进行抛物线惩罚
/// ancestal_home同current_place
/// economic存款，车，房，也按照正向小惩罚，反向大惩罚
/// height，weight不匹配者均相同惩罚
/// original_family_composition，parentts_situation根据标签对比进行惩罚

#[derive(Deserialize, Serialize, Debug)]
pub struct Candidate {
    birth_year: Option<i8>,//实际年龄
    work: Option<Vec<i8>>,//按照包含关系，填入编号
    qualification: Option<i8>,//学历编号1-6，
    current_place: Option<Vec<i8>>,//按照包含关系，填入编号
    ancestal_home: Option<Vec<i8>>,//按照包含关系，填入编号
    economic: Option<f64>,//实际财富
    height: Option<f64>,//实际身高
    weight: Option<f64>,//实际体重
    score: f64,
}

async fn set_score(
    mut candidates: Vec<Candidate>,
    candidate: Candidate,
    attributes: HashMap<String, f64>,
) -> Result<Vec<Candidate>, Error> {
    for c in &mut candidates {
        c.score = calculate_total_score(c, &candidate, &attributes)?;
    }
    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(candidates)
}

fn calculate_total_score(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    let mut total_score = 0.0;
    let properties = vec![
        "birth_year",
        "work",
        "qualification",
        "current_place",
        "ancestal_home",
        "economic",
        "height",
        "weight",
    ];
    for property_name in properties {
        let score_function = scoring_rules::get_score_function(property_name).unwrap();
        total_score += score_function(candidate, candidate_condition, attributes)?;
    }
    Ok(total_score)
}
