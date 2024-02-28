use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_birth_year(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(birth_year_condition) = &candidate_condition.birth_year {
        let weights_score = attributes.get("birth_year").ok_or(Error)?;
        match &candidate.birth_year {
            None => return Ok(weights_score / 2.0),
            Some(birth_year) => {
                let difference = (birth_year - birth_year_condition).abs() as f64;
                let score = parabola(difference, weights_score, 5f64);
                if score < 0.0 {
                    return Ok(0.0);
                }
                return Ok(score);
            }
        }
    }
    Ok(0.0)
}

//a是权重分数
//x是年龄差或者身高差
//高的时候的惩罚到b = 15
//矮的时候的惩罚到b = 10
//惩罚函数
pub fn parabola(x: f64, a: &f64, b: f64) -> f64 {
    a * (x - b).powi(2)
}
