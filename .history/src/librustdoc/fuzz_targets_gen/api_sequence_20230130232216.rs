use rustc_hir::def;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty, TyCtxt};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Api<'tcx> {
    pub full_name: String,
    pub def_id: DefId,
    pub params: Vec<Param<'tcx>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Param<'tcx> {
    pub ty: Ty<'tcx>,                          //参数类型
    pub returned_by_full_name: Option<String>, //被哪个参数返回
    pub index: usize,                          //在参数列表里的位置
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApiSequence<'tcx> {
    pub crate_name: String,       //crate名字
    pub sequence: Vec<Api<'tcx>>, //函数序列
}

impl ApiSequence<'_> {
    pub fn new_from_function_full_names(
        crate_name: String,
        full_names: Vec<String>,
        tcx: TyCtxt<'_>,
    ) -> Self {
        for full_name in full_names {
            //找到对应的函数
            let local_def_id = tcx.hir().body_owners().find(|x| match tcx.def_kind(*x) {
                def::DefKind::Fn
                | def::DefKind::AssocFn
                | def::DefKind::Closure
                | def::DefKind::Generator => {
                    let name = crate_name + "::" + &tcx.def_path_str(x.to_def_id());
                    name == full_name
                }
                _ => false,
            });

            let local_def_id = match local_def_id {
                Some(id) => id,
                None => {
                    panic!("Didn't find function {}", full_name);
                }
            };
        }
        ApiSequence { crate_name: "123".to_string(), sequence: () }
    }
}
