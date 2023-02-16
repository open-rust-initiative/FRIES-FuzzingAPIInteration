use rustc_hir::def_id::DefId;
use rustc_middle::ty::Ty;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Api<'tcx> {
    pub full_name: String,
    pub def_id: DefId,
    pub params: Vec<Param<'tcx>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Param<'tcx> {
    pub ty: Ty<'tcx>,
    pub returned_by: Option<&'tcx Api<'tcx>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ApiSequence<'a> {
    pub kcrate_name: String,
    pub function_sequence: Vec<Api<'a>>,
}
