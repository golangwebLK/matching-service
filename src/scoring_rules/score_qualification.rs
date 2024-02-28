use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;
//学历分为6阶：小学1，初中2，高中3，本科4，硕士5，博士6
pub fn score_qualification(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(qualification_condition) = &candidate_condition.qualification {
        let weights_score = attributes.get("qualification").ok_or(Error)?;
        match &candidate.qualification {
            None => return Ok(weights_score / 2.0),
            Some(qualification) => {
                let difference = (qualification - qualification_condition).abs() as f64;
                let score = parabola(difference, weights_score, 5f64);
                return Ok(score);
            }
        }
    }
    Ok(0.0)
}
