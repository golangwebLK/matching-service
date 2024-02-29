use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_weight(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(weight_condition) = &candidate_condition.weight {
        let weights_score = attributes.get("weight").ok_or(Error)?;
        match &candidate.weight {
            None => return Ok(weights_score / 2.0),
            Some(weight) => {
                let difference = (weight - weight_condition).abs();
                let score = parabola(difference, weights_score, 15f64);
                if score < 0.0 {
                    return Ok(0.0);
                }
                return Ok(score);
            }
        }
    }
    Ok(0.0)
}
