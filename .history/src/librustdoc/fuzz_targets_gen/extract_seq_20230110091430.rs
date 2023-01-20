use crate::fuzz_targets_gen::extract_dep::AllDependencies;
use crate::fuzz_targets_gen::util::{Node, Stack};
use rustc_middle::mir;
use rustc_middle::ty::{self, Ty, TyCtxt};

/// 进行一个深度优先搜索，然后生成遍历序列
/// 获得函数签名之后，就获得了生成序列的源信息
pub fn _extract_sequence<'tcx>(tcx: TyCtxt<'tcx>, all_dependencies: AllDependencies<'tcx>) {
    for (caller, function) in all_dependencies.functions {
        //FIXME:
        if let Some(caller_local) = caller.as_local() {
            let mir = tcx.mir_built(ty::WithOptConstParam {
                did: caller_local,
                const_param_did: tcx.opt_const_param_of(caller_local),
            });
            let mir = mir.borrow();
            let mir: &mir::Body<'_> = &mir;
        }
    }
}
