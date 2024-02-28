mod scoring_rules;

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


///打分策略
/// birth_year 通过年龄匹配，正好合适就给出所有权重分数，
/// 如果是男生大于女生的年轻惩罚分数要小，男生小于女生惩罚分数要大一些
/// work传入一个工作栈，出栈即为上一层工作类型，每上一层惩罚分数就会抛物线型加大
/// qualification 学历如果高于匹配学历进行小惩罚，低于匹配学历进行大惩罚
/// current_place传入一个地区栈，出栈顺序区，市，省。逐层进行抛物线惩罚
/// ancestal_home同current_place
/// economic存款，车，房，也按照正向小惩罚，反向大惩罚
/// height，weight不匹配者均相同惩罚
/// original_family_composition，parentts_situation根据标签对比进行惩罚

#[derive(Deserialize,Serialize, Debug)]
pub struct Candidate {
    gender: Option<String>,
    birth_year: Option<i8>,
    work: Option<Vec<i8>>,
    qualification: Option<String>,
    current_place: Option<Vec<i8>>,
    ancestal_home: Option<Vec<i8>>,
    economic: Option<Vec<String>>,
    height: Option<i32>,
    weight: Option<i32>,
    original_family_composition: Option<Vec<String>>,
    parentts_situation: Option<Vec<String>>,
    score: f64,
}

async fn set_score(mut candidates: Vec<Candidate>, attributes: HashMap<String,f64>) -> Result<Vec<Candidate>,Error>{
    for candidate in &mut candidates {
        candidate.score = calculate_total_score(candidate, &attributes);
    }
    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(candidates)
}

fn calculate_total_score(candidate: &Candidate, attributes: &HashMap<String, f64>) -> f64 {
    let mut total_score = 0.0;
    if let Some(birth_year) = &candidate.birth_year{
        total_score += scoring_rules::score_birth_year::score_birth_year(birth_year,&candidate.gender,attributes);
    }
    if let Some(work) = &candidate.work{
        total_score += scoring_rules::score_work::score_work(work,attributes);
    }
    if let Some(qualification) = &candidate.qualification{
        total_score += scoring_rules::score_qualification::score_qualification(qualification,attributes);
    }
    if let Some(current_place) = &candidate.current_place{
        total_score += scoring_rules::score_current_place::score_current_place(current_place,attributes);
    }
    if let Some(ancestal_home) = &candidate.ancestal_home{
        total_score += scoring_rules::score_ancestal_home::score_ancestal_home(ancestal_home,attributes);
    }
    if let Some(economic) = &candidate.economic{
        total_score += scoring_rules::score_economic::score_economic(economic,attributes);
    }
    if let Some(height) = &candidate.height{
        total_score += scoring_rules::score_height::score_height(height,attributes);
    }
    if let Some(weight) = &candidate.weight{
        total_score += scoring_rules::score_weight::score_weight(weight,attributes);
    }
    if let Some(original_family_composition) = &candidate.original_family_composition{
        total_score += scoring_rules::score_original_family_composition::score_original_family_composition(original_family_composition,attributes);
    }
    if let Some(parentts_situation) = &candidate.parentts_situation{
        total_score += scoring_rules::score_parentts_situation::score_parentts_situation(parentts_situation,attributes);
    }
    total_score
}

