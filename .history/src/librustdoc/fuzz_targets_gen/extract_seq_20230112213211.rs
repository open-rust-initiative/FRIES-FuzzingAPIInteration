use crate::fuzz_targets_gen::extract_dep::AllDependencies;
use crate::fuzz_targets_gen::util::Stack;
use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::mir;
use rustc_middle::ty::{self, Ty, TyCtxt};

use super::extract_dep::{extract_arguments, Argument, CalleeDependency, Function};

pub struct ExtractSequence {
    all_sequence: Vec<Vec<String>>,
}

impl ExtractSequence {
    pub fn new() -> Self {
        ExtractSequence { all_sequence: Vec::new() }
    }

    /// 进行一个深度优先搜索，然后生成遍历序列
    /// 获得函数签名之后，就获得了生成序列的源信息
    pub fn extract_sequence<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        all_dependencies: AllDependencies<'tcx>,
        enable: bool,
    ) {
        // 装入所有解析的序列
        let mut all_seq = Vec::new();
        let mut visit_set = FxHashSet::default();

        if !enable {
            return;
        }

        //遍历每一个函数
        for (caller_def_id, function) in all_dependencies.functions {
            if caller_def_id.is_local() {
                // 测试每一个参数，如果有任何一个不是primitive类型的，都会成功
                let args: Vec<Argument<'_>> = function.arguments;
                if args.iter().all(|arg| arg.ty.is_primitive_ty()) {
                    // 能进入这里，说明参数都是基本类型，说明是我们的起始节点
                    // 下面开始dfs

                    let mut func_seq = Vec::new();
                    let mut stack = Stack::new();

                    //dfs的start node，初始化stack
                    let function_info = FunctionInfo::new_by_caller_def_id(tcx, caller_def_id);
                    stack.push(function_info);

                    // 开始进行dfs，是用栈来避免递归
                    while !stack.is_empty() {
                        let function_info = stack.pop().unwrap();

                        //下面对于每一个孩子进行遍历
                        let callee_dependency = function.callee_dependencies.clone();
                        for CalleeDependency { callee, callsite, .. } in callee_dependency {
                            use super::extract_dep::Callee;
                            let (crate_name, def_id) = match callee {
                                Callee::DirectCall(def_id) => {
                                    (tcx.crate_name(caller.krate).as_str().to_string(), def_id)
                                }
                                Callee::LocalFunctionPtr(_) => continue, //跳过
                            };

                            if crate_name == "url" {
                                // FIXME: 如果是test crate的api，推入序列
                                func_seq.push(tcx.def_path_str(def_id));

                                if callsite.return_ty.is_primitive_ty() {
                                    all_seq.push(func_seq.clone());
                                    func_seq.clear();
                                }
                                continue;
                            }

                            //只有本地函数才能被推入stack
                            match def_id.as_local() {
                                Some(_) => {
                                    if visit_set.contains(&def_id) {
                                        println!("find!!!!!!!!!!!!!!!!!!!");
                                        continue;
                                    }
                                    visit_set.insert(def_id);
                                    // 否则存入stack供下次遍历
                                    stack.push(def_id);
                                }
                                None => {}
                            }
                        }
                    }

                    // dfs完毕，开始进行结束处理
                    all_seq.push(func_seq);

                    // 结束
                }
            }
        }

        self.all_sequence = all_seq;
    }

    pub fn print_sequence(self) {
        for (idx, seq) in self.all_sequence.iter().enumerate() {
            print!("Sequence {}: ", idx);
            for func in seq {
                print!("{} ", func);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionInfo<'tcx> {
    is_caller_def_id: bool,
    def_id: DefId,
    mir: mir::Body<'tcx>,
    return_ty: Ty<'tcx>,
    arguments: Vec<Argument<'tcx>>,
    dependency_info: Function<'tcx>,
}

#[allow(dead_code)]
impl FunctionInfo<'_> {
    /// 对于当前crate的LocalDefId
    pub fn new_by_caller_def_id<'tcx>(
        tcx: TyCtxt<'tcx>,
        def_id: DefId,
        all_dependencies: AllDependencies<'tcx>,
    ) -> FunctionInfo<'tcx> {
        let local_def_id = match def_id.as_local() {
            Some(local_id) => local_id,
            None => {
                panic!("This is not a caller in this crate")
            }
        };

        let mir = tcx.mir_built(ty::WithOptConstParam {
            did: local_def_id,
            const_param_did: tcx.opt_const_param_of(local_def_id),
        });
        let mir = mir.borrow();
        let mir: &mir::Body<'_> = &mir;
        let mir = mir.to_owned();

        // 返回值
        let return_ty = mir.local_decls[mir::Local::from_usize(0)].ty;
        // 参数
        let arguments = extract_arguments(&mir);

        let dependency_info = all_dependencies
            .functions
            .iter()
            .find(|(x, _)| tcx.def_path_str(*x) == tcx.def_path_str(def_id))
            .unwrap()
            .1;

        FunctionInfo { is_caller_def_id: true, def_id, mir, return_ty, arguments, dependency_info }
    }

    pub fn new_by_callee_def_id<'tcx>(
        tcx: TyCtxt<'tcx>,
        def_id: DefId,
        all_dependencies: AllDependencies<'tcx>,
    ) -> FunctionInfo<'tcx> {
        // 获得调用点def_id对应的function的local_def_id
        let local_def_id = match tcx
            .hir()
            .body_owners()
            .find(|x| tcx.def_path_str(x.to_def_id()) == tcx.def_path_str(def_id))
        {
            //当callee的路径和body_owner一样的时候，就可以找到函数体
            Some(local) => local,
            None => {
                panic!("No such a callee in this crate");
            }
        };

        //获取mir::Body
        let mir = tcx.mir_built(ty::WithOptConstParam {
            did: local_def_id,
            const_param_did: tcx.opt_const_param_of(local_def_id),
        });
        let mir = mir.borrow();
        let mir: &mir::Body<'_> = &mir;
        let mir = mir.to_owned();

        // caller
        let def_id = local_def_id.to_def_id();
        // 返回值
        let return_ty = mir.local_decls[mir::Local::from_usize(0)].ty;
        // 参数
        let arguments = extract_arguments(&mir);

        let dependency_info = all_dependencies
            .functions
            .iter()
            .find(|(x, _)| tcx.def_path_str(*x) == tcx.def_path_str(def_id));

        FunctionInfo { is_caller_def_id: true, def_id, mir, return_ty, arguments }
    }
}
