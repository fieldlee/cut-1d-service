use crate::models::{CutSolver,ResultSolver,SubResultSolver};
use std::result::Result;
use cut_optimizer_1d::Optimizer;
use cut_optimizer_1d::{StockPiece,CutPiece};

// 计算最优方案
pub fn solve_cut_optimize_for_service( slover: CutSolver) -> Result<ResultSolver,cut_optimizer_1d::Error> {
    let mut result_info = ResultSolver::default();
    let mut solution = Optimizer::new();
    // 添加母卷的信息
    for ele in slover.parent_rolls().iter() {
        solution.add_stock_piece(StockPiece{
            length:ele.width-slover.out_side,
            weight:ele.weight,
            quantity:Some(ele.quantity),
        });
    }
    // 添加子卷的信息
    for (index,ele)  in slover.child_rolls().iter().enumerate() {
        solution.add_cut_piece(CutPiece{
            length:ele.width,
            external_id:Some(index),
            quantity:ele.quantity,
        });
    }
    // 计算切割方案
   let optimizer = solution
    .set_cut_width(slover.side)
    .set_random_seed(slover.seed as u64)
    .allow_mixed_stock_sizes(true)
    .optimize(|_| {});

    match optimizer  {
        Ok(optimizer) => {
            let mut result_cut_pieces = vec![];
            let mut used_stock = 0;
            // 设置子卷长度数组
            for ele in slover.child_rolls().iter() {
                result_info.sub_weights.push(0);
            }
            // 循环计算切割捆包的值
            for ele in optimizer.stock_pieces {
                used_stock += 1;
                let mut e_cut_piece = SubResultSolver::default();
                let mut remain_length = ele.length;
                let mut all_sub_len = vec![];
                for e in ele.cut_pieces {
                    all_sub_len.push(e.length);
                    result_info.sub_weights[e.external_id.unwrap()] += e.weight;
                    remain_length -= e.length;
                }
                e_cut_piece.set_un_used(remain_length);
                let un_used_weight =  ((remain_length as f32 / ele.length as f32) * ele.weight as f32) as usize;
                e_cut_piece.set_un_used_weight(un_used_weight);
                e_cut_piece.set_subs(all_sub_len);
                result_cut_pieces.push(e_cut_piece);
            }
            // 赋值 返回
            result_info.set_status_name("OPTIMAL".to_string());
            result_info.set_num_unique_solutions("1".to_string());
            result_info.set_num_solutions("1".to_string());
            result_info.set_num_rolls_used(used_stock);
            result_info.set_solutions(result_cut_pieces);
            return Ok(result_info);
        },
        Err(err) => {
            return Err(err);
        }
    }
}
