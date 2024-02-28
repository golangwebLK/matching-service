use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_current_place(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(current_place_condition) = &candidate_condition.current_place {
        let weights_score = attributes.get("current_place").ok_or(Error)?;
        match &candidate.current_place {
            None => return Ok(weights_score / 2.0),
            Some(current_place) => {
                let length = current_place.len();
                match length {
                    1 => {
                        if current_place[0] != current_place_condition[0] {
                            return Ok(0.0);
                        }
                        return Ok(weights_score.clone());
                    }
                    2 => {
                        if current_place[0] != current_place_condition[0] {
                            return Ok(0.0);
                        }
                        if current_place[1] != current_place_condition[1] {
                            let score = parabola(1f64, weights_score, 2f64);
                            return Ok(score);
                        }
                        return Ok(weights_score.clone());
                    }
                    3 => {
                        if current_place[0] != current_place_condition[0] {
                            return Ok(0.0);
                        }
                        if current_place[1] != current_place_condition[1] {
                            let score = parabola(2f64, weights_score, 3f64);
                            return Ok(score);
                        }
                        if current_place[2] != current_place_condition[2] {
                            let score = parabola(1f64, weights_score, 3f64);
                            return Ok(score);
                        }
                        return Ok(weights_score.clone());
                    }
                    _ => return Ok(0.0),
                }
            }
        }
    }
    Ok(0.0)
}
