use rustc_hash::FxHashSet;

// 生态系统内部的图集合
struct BigGraph {
    crate_graph_set: FxHashSet<CrateGraph>,
}

// 对于单个crate，内部api
struct CrateGraph {
    krate: CrateNum,
}
