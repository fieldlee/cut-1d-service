use getset::*;
use serde::{Deserialize, Serialize};
use http::{StatusCode};
use axum::Json;
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CutLenSolver {
    pub child_rolls: Vec<SubChildLenSolver>,
    pub parent_rolls: Vec<SubParentSolver>,
    pub side: usize,
    pub out_side: usize,
    pub seed: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubChildLenSolver {
   pub width: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CutSolver {
    pub child_rolls: Vec<SubChildSolver>,
    pub parent_rolls: Vec<SubParentSolver>,
    pub side: usize,
    pub out_side: usize,
    pub seed: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ChildSolverList {
   pub solutions: Vec<RuleSolutions> ,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubChildSolver {
   pub quantity: usize,
   pub width: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubParentSolver {
   pub quantity: usize,
   pub width: usize,
   pub weight: usize,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ResultSolver {
    pub status_name: String,
    pub num_solutions: String,
    pub num_unique_solutions: String,
    pub num_rolls_used: i8,
    pub solutions: Vec<SubResultSolver>,
    pub sub_weights: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubResultSolver {
    un_used: usize,
    un_used_weight: usize,
    subs: Vec<usize>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ResultSolverList {
    solutions:Vec<ResultSolver>
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RuleSolutions {
    sub_child_solver: Vec<SubChildSolver>,
    solutions:ResultSolver,
}

type OptimizeError = (StatusCode, Json<Value>);

pub fn error(status_code: StatusCode, message: &str) -> OptimizeError {
    (status_code, Json(json!({ "message": message })))
}

pub fn error_with_data<T: Serialize>(status_code: StatusCode, message: &str, data: T) -> OptimizeError {
    (
        status_code,
        Json(json!({ "message": message, "data": data })),
    )
}