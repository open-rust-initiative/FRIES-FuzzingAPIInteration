use crate::fuzz_targets_gen::extract_dep::AllDependencies;

/// 进行一个深度优先搜索，然后生成遍历序列
/// 获得函数签名之后，就获得了生成序列的源信息
pub fn _extract_sequence<'tcx>(all_dependencies: AllDependencies<'tcx>) {
    for _caller in all_dependencies.functions {
        //FIXME:
    }
}
