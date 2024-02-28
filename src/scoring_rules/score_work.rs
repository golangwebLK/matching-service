use crate::scoring_rules::score_birth_year::parabola;
use crate::Candidate;
use std::collections::HashMap;
use std::fmt::Error;

pub fn score_work(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>,
) -> Result<f64, Error> {
    if let Some(work_condition) = &candidate_condition.work {
        let weights_score = attributes.get("work").ok_or(Error)?;
        match &candidate.work {
            None => return Ok(weights_score / 2.0),
            Some(work) => {
                let length = work.len();
                match length {
                    1 => {
                        if work[0] != work_condition[0] {
                            return Ok(0.0);
                        }
                        return Ok(weights_score.clone());
                    }
                    2 => {
                        if work[0] != work_condition[0] {
                            return Ok(0.0);
                        }
                        if work[1] != work_condition[1] {
                            let score = parabola(1f64, weights_score, 2f64);
                            return Ok(score);
                        }
                        return Ok(weights_score.clone());
                    }
                    3 => {
                        if work[0] != work_condition[0] {
                            return Ok(0.0);
                        }
                        if work[1] != work_condition[1] {
                            let score = parabola(2f64, weights_score, 3f64);
                            return Ok(score);
                        }
                        if work[2] != work_condition[2] {
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
