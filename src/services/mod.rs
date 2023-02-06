use crate::models::{CutSolver,CutLenSolver, ResultSolver,SubResultSolver,SubChildSolver,ChildSolverList,error,RuleSolutions};
use crate::utils::error::Result;
use cut_optimizer_1d::Optimizer;
use cut_optimizer_1d::{StockPiece,CutPiece};
use random_number::random;
use std::ops::Index;
use std::rc::Rc;
use std::cell::RefCell;
use http::{StatusCode};
use crate::utils::cut::solve_cut_optimize_for_service;
pub struct CutSolverService;
impl Default for CutSolverService {
    fn default() -> Self {
        CutSolverService {}
    }
}
impl CutSolverService {
    // 计算最优方案
    pub async fn solve_cut_optimize(&self, slover: CutSolver) -> Result<ResultSolver> {
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
            if ele.quantity > 0 {
                solution.add_cut_piece(CutPiece{
                    length:ele.width,
                    external_id:Some(index),
                    quantity:ele.quantity,
                });
            }
        }
        // 计算切割方案
       let optimizer = solution
        .set_cut_width(slover.side)
        .set_random_seed(slover.seed as u64)
        .allow_mixed_stock_sizes(true)
        .optimize(|_| {})
        .map_err(|e| match e {
            cut_optimizer_1d::Error::NoFitForCutPiece(cut_piece) => error(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Cut piece doesn't fit in any stock pieces",
            ),
        });
        match optimizer {
            Ok(solution) => {
                let mut result_info = ResultSolver::default();
                let mut result_cut_pieces = vec![];
                let mut used_stock = 0;
                // 设置子卷长度数组
                for ele in slover.child_rolls().iter() {
                    result_info.sub_weights.push(0);
                }
                // 循环计算切割捆包的值
                for ele in solution.stock_pieces {
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
                let mut result_info = ResultSolver::default();
                result_info.set_status_name("Error".to_string());
                return Ok(result_info);
            },
        }
    }

    //计算数量
    pub async fn com_optimize_quantity(&self,len_solver: &CutLenSolver) -> Result<Vec<CutSolver>> {

        let parent_all_valid_len = len_solver.parent_rolls.iter().map(|parent|{
            parent.width() * parent.quantity()
        }).sum::<usize>() - (len_solver.out_side() * len_solver.parent_rolls().len());
    
        let parent_all_valid_len_min = parent_all_valid_len - len_solver.parent_rolls.len()*8;
        
        // 计算子卷的最大数量
        let mut childs_quantity = vec![];
        let  childs_quantity_zero:Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(vec![0;len_solver.child_rolls.len()]));
        let mut con_childs_quantity:Vec<Vec<usize>> = vec![vec![]];
        for ele in len_solver.child_rolls.iter(){
            childs_quantity.push( (parent_all_valid_len as f32 / ele.width as f32) as usize );
        }
        println!("childs_quantity: {:?}", childs_quantity);
        // 最多循环1000次
        for i in 0..1000{
            let mut rng = random_number::rand::thread_rng();
            let n: usize = random!(0..childs_quantity.len(), rng);
            
            let tmp1 = childs_quantity_zero.borrow_mut().index(n).clone();
            childs_quantity_zero.borrow_mut()[n] = tmp1 + 1;
            // 如果 大于最大的有效数量继续
            let tmp = childs_quantity_zero.borrow_mut().index(n).clone();
            if tmp >= childs_quantity[n] -1 {
                childs_quantity_zero.borrow_mut()[n] -= 1;
                continue;
            }

            // 计算子卷宽度 的长度 不小于 差额的5mm
            let mut com_childs_width_all = 0;
            println!("childs_quantity_zero:{:?}", childs_quantity_zero);
            for (index,quantity) in childs_quantity_zero.borrow_mut().iter().enumerate(){
                com_childs_width_all += len_solver.child_rolls[index].width * quantity;
            }
            // 在有效的长度内计入有效的列表
            if  parent_all_valid_len_min  < com_childs_width_all &&  com_childs_width_all <= parent_all_valid_len {
                con_childs_quantity.push(childs_quantity_zero.borrow_mut().to_vec());
            }
        }
        println!("con_childs_quantity:{:?}",con_childs_quantity);
        let mut childs = vec![];

        for ele  in con_childs_quantity.iter() {
            let cut_solver = CutSolver::default();

            childs.push(cut_solver)
        }

        return Ok(childs);

    }

    // 碰撞计算可能满足条件的切割方案
    pub async fn com_optimize_quantity_by_len(&self,len_solver: &CutLenSolver) -> Result<ChildSolverList> {
        println!("len_solver:{:?}",len_solver);
        let parent_all_valid_len = len_solver.parent_rolls.iter().map(|parent|{
            parent.width() * parent.quantity()
        }).sum::<usize>() - (len_solver.out_side() * len_solver.parent_rolls().len());
    
        let parent_all_valid_len_min = parent_all_valid_len - len_solver.parent_rolls.len()*10000;
        
        // 计算子卷的最大数量
        let mut childs_quantity = vec![];
        let  childs_quantity_zero = Rc::new(RefCell::new(vec![0;len_solver.child_rolls.len()]));
        let mut con_childs_quantity = vec![];
        for ele in len_solver.child_rolls.iter(){
            childs_quantity.push( (parent_all_valid_len as f32 / ele.width as f32) as usize );
        }
        // 最多循环1000次
        for i in 0..1000000{
            for index in 0..childs_quantity.len(){
                let mut rng = random_number::rand::thread_rng();
                let n: usize = random!(0..childs_quantity.index(index), rng);
                childs_quantity_zero.borrow_mut()[index]=n;
            }
            // 计算子卷宽度 的长度 不小于 差额的5mm
            let mut com_childs_width_all = 0;
            for (index,quantity) in childs_quantity_zero.borrow_mut().iter().enumerate(){
                com_childs_width_all += len_solver.child_rolls[index].width * quantity;
            }
            // 在有效的长度内计入有效的列表
            
            if  parent_all_valid_len_min  < com_childs_width_all &&  com_childs_width_all <= parent_all_valid_len {
                println!("parent_all_valid_len_min:{},parent_all_valid_len:{}",parent_all_valid_len_min,parent_all_valid_len);
                if !con_childs_quantity.contains(childs_quantity_zero.borrow_mut().to_vec().as_ref()){
                    con_childs_quantity.push(childs_quantity_zero.borrow_mut().to_vec());
                }
            }
        }
        let mut result_list = vec![];
        let mut result_child_solvers = ChildSolverList::default();
        if con_childs_quantity.len() > 0 {
            for ele in con_childs_quantity.iter() {
                let  mut childs = vec![];
                for (index,quantity) in ele.iter().enumerate() {
                    let mut sub_child = SubChildSolver::default();
                    sub_child.quantity = *quantity;
                    sub_child.width = len_solver.child_rolls.index(index).width().to_owned();
                    childs.push(sub_child);
                }
                result_list.push(childs);
            }
        }
        let mut valid_result_list:Vec<RuleSolutions> = vec![];
        // 检查计算结果是否有可行解
        if result_list.len() > 0 {
            for ele in result_list.iter() {
                let mut cut_solver_tmp = CutSolver::default();
                let mut rule_solutions = RuleSolutions::default();
                cut_solver_tmp.set_child_rolls(ele.to_vec());
                cut_solver_tmp.set_parent_rolls(len_solver.parent_rolls().to_vec());
                cut_solver_tmp.set_out_side(*(len_solver.out_side()));
                cut_solver_tmp.set_side(*(len_solver.side()));
                cut_solver_tmp.set_seed(*(len_solver.seed()));
                let cut_solver_result_tmp = solve_cut_optimize_for_service(cut_solver_tmp);
                match cut_solver_result_tmp {
                    Ok(result) => {
                        // println!("result.status_name():{:?}",result.status_name());
                        if result.status_name() != "Error" {
                            rule_solutions.set_sub_child_solver(ele.to_vec());
                            rule_solutions.set_solutions(result);
                            valid_result_list.push(rule_solutions);
                        }
                    },
                    Err(err) =>{
                        println!("error:{:?}",err)
                    },
                }
            }
        }
        result_child_solvers.set_solutions(valid_result_list);
        return Ok(result_child_solvers);
    }
}



            //     let mut childs_solver = vec![];
            //     for (index,quantity) in ele.iter().enumerate() {{
            //         let mut sub_child_solver = SubChildSolver::default();
            //         sub_child_solver.quantity = *quantity;
            //         sub_child_solver.width = len_solver.child_rolls[index].width;
            //         childs_solver.push(sub_child_solver);
            //     }}
                
            //     let mut cut_solver = CutSolver::default();
            //     cut_solver.child_rolls = childs_solver;
            //     cut_solver.parent_rolls = len_solver.parent_rolls().to_vec();
            //     cut_solver.out_side = len_solver.out_side;
            //     cut_solver.seed = len_solver.seed;

            //     println!("solve_cut_optimize_for_service cut_solver : {:?}", cut_solver);
            //     // 处理返回结果
            //     match solve_cut_optimize_for_service(cut_solver) {
            //         Ok(result) => {
            //             result_list.push(result);
            //         },
            //         Err(err) => {
            //             println!("solve_cut_optimize_for_service error : {:?}", err);
            //             continue;
            //         }
            //     }
            // }
            // result_solver_list.set_solutions(result_list);