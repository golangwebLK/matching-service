use std::collections::HashMap;
use std::fmt::Error;
use crate::Candidate;
use crate::scoring_rules::score_birth_year::parabola;

pub fn score_economic(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>
) -> Result<f64,Error> {
    if let Some(economic_condition) = &candidate_condition.economic {
        let weights_score = attributes.get("economic").ok_or(Error)?;
        match &candidate.economic {
            None => return Ok(weights_score / 2.0),
            Some(economic) => {
                let difference = (economic - economic_condition).abs() as f64;
                let b = economic * 0.3;
                let score = parabola(difference, weights_score, b);
                if score < 0.0 {
                    return Ok(0.0);
                }
                return Ok(score);
            }
        }
    }
    Ok(0.0)
}