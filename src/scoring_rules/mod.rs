use std::collections::HashMap;
use std::fmt::Error;
use crate::Candidate;

//年龄打分规则
mod score_birth_year;
//工作打分规则
mod score_work;
//学历打分规则
mod score_qualification;
//当前住址打分规则
mod score_current_place;
//祖籍打分规则
mod score_ancestal_home;
//经济状况打分规则
mod score_economic;
//身高打分规则
mod score_height;
//体重打分规则
mod score_weight;
//原生家庭状况打分规则
mod score_original_family_composition;
//父母状况打分规则
mod score_parents_situation;


pub fn get_score_function(
    property_name: &str
) -> Result<fn(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>
) -> Result<f64, Error>, std::io::Error> {
    match property_name {
        "birth_year" => Ok(score_birth_year::score_birth_year),
        "work" => Ok(score_work::score_work),
        "qualification" => Ok(score_qualification::score_qualification),
        "current_place" => Ok(score_current_place::score_current_place),
        "ancestal_home" => Ok(score_ancestal_home::score_ancestal_home),
        "economic" => Ok(score_economic::score_economic),
        "height" => Ok(score_height::score_height),
        "weight" => Ok(score_weight::score_weight),
        "original_family_composition" => Ok(score_original_family_composition::score_original_family_composition),
        "parents_situation" => Ok(score_parents_situation::score_parents_situation),
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other,"没有找到对应属性"))
    }
}

