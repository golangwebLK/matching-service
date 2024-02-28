use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_height(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(height_condition) = &candidate_condition.height {
        let weights_score = attributes.get("height").ok_or(Error)?;
        match &candidate.height {
            None => return Ok(weights_score / 2.0),
            Some(height) => {
                let difference = (height - height_condition).abs() as f64;
                let score = parabola(difference, weights_score, 10f64);
                if score < 0.0 {
                    return Ok(0.0);
                }
                return Ok(score);
            }
        }
    }
    Ok(0.0)
}
