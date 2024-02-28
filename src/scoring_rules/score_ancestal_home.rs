use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_ancestal_home(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(ancestal_home_condition) = &candidate_condition.ancestal_home {
        let weights_score = attributes.get("ancestal_home").ok_or(Error)?;
        match &candidate.ancestal_home {
            None => return Ok(weights_score / 2.0),
            Some(ancestal_home) => {
                let length = ancestal_home.len();
                match length {
                    1 => {
                        if ancestal_home[0] != ancestal_home_condition[0] {
                            return Ok(0.0);
                        }
                        return Ok(weights_score.clone());
                    }
                    2 => {
                        if ancestal_home[0] != ancestal_home_condition[0] {
                            return Ok(0.0);
                        }
                        if ancestal_home[1] != ancestal_home_condition[1] {
                            let score = parabola(1f64, weights_score, 2f64);
                            return Ok(score);
                        }
                        return Ok(weights_score.clone());
                    }
                    3 => {
                        if ancestal_home[0] != ancestal_home_condition[0] {
                            return Ok(0.0);
                        }
                        if ancestal_home[1] != ancestal_home_condition[1] {
                            let score = parabola(2f64, weights_score, 3f64);
                            return Ok(score);
                        }
                        if ancestal_home[2] != ancestal_home_condition[2] {
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
