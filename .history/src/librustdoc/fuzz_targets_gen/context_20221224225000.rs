use std::rc::Rc;

//use rustc_data_structures::fx::{FxHashMap, FxHashSet};
//use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

use crate::clean::types as clean_types;
use crate::config::RenderOptions;
use crate::error::Error;
use crate::formats::cache::Cache;
use crate::formats::FormatRenderer;

#[derive(Clone)]
pub(crate) struct Context<'tcx> {
    /// Current hierarchy of components leading down to what's currently being
    /// rendered
    pub(crate) current: Vec<Symbol>,

    /// Type Context
    pub(crate) _tcx: TyCtxt<'tcx>,
    pub(crate) _cache: Rc<Cache>,
}

impl Context<'_> {
    pub(crate) fn full_path(self, item: &types::item) -> String {
        fn join_with_double_colon(syms: &[Symbol]) -> String {
            let mut s = String::with_capacity(estimate_item_path_byte_length(syms.len()));
            s.push_str(syms[0].as_str());
            for sym in &syms[1..] {
                s.push_str("::");
                s.push_str(sym.as_str());
            }
            s
        }

        let mut s = join_with_double_colon(&cx.current);
        s.push_str("::");
        s.push_str(item.name.unwrap().as_str());
        s
    }
}

impl<'tcx> FormatRenderer<'tcx> for Context<'tcx> {
    fn descr() -> &'static str {
        "fuzz targets generator"
    }

    const RUN_ON_MODULE: bool = true;

    fn init(
        krate: clean::Crate,
        _options: RenderOptions,
        cache: Cache,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(Self, clean::Crate), Error> {
        println!("Name of the parsed crate is {}.", krate.name(tcx));

        Ok((Context { current: Vec::new(), _tcx: tcx, _cache: Rc::new(cache) }, krate))
    }

    fn make_child_renderer(&self) -> Self {
        self.clone()
    }

    fn item(&mut self, _item: clean::Item) -> Result<(), Error> {
        todo!()
    }

    fn mod_item_in(&mut self, item: &clean::Item) -> Result<(), Error> {
        let item_name = item.name.unwrap();
        self.current.push(item_name);

        //TODO:
        Ok(())
    }

    fn mod_item_out(&mut self) -> Result<(), Error> {
        info!("mod_item_out");

        // Go back to where we were at
        self.current.pop();
        Ok(())
    }

    fn after_krate(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn cache(&self) -> &Cache {
        &self._cache
    }
}
