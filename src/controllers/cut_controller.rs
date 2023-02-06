
use crate::models::{CutSolver,CutLenSolver};
use crate::services::CutSolverService;
use crate::utils::RespVO;
use crate::APPLICATION_CONTEXT;
use axum::response::IntoResponse;
use axum::Json;


//安装重量切割方案
pub async fn solve_cut_optimize_by_weight(Json(solver): Json<CutSolver>) -> impl IntoResponse {
    let cut_service = APPLICATION_CONTEXT.get::<CutSolverService>();

    let result = cut_service.solve_cut_optimize(solver).await;

    println!("result:{:?}",result);
    
    return RespVO::from_result(&result).resp_json();
}

//安装长度切割方案
pub async fn solve_cut_optimize_by_len(Json(solver): Json<CutLenSolver>) -> impl IntoResponse {
    let cut_service = APPLICATION_CONTEXT.get::<CutSolverService>();

    let result = cut_service.com_optimize_quantity_by_len(&solver).await;

    return RespVO::from_result(&result).resp_json();
}