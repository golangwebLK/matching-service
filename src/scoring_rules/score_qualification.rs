use std::collections::HashMap;
use std::fmt::Error;
use crate::Candidate;

pub fn score_qualification(
    candidate: &Candidate,
    candidate_condition: &Candidate,
    attributes: &HashMap<String, f64>
) -> Result<f64,Error> {
    Ok(0.0)
}