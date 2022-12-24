use crate::clean::types as clean_types;

#[derive(Clone, Debug)]
pub struct Function {
    //函数声明FnDecl，包含参数和返回值类型
    pub(crate) inputs: Vec<clean_types::Argument>,
    pub(crate) output: clean_types::FnRetTy,
    pub(crate) c_variadic: bool,

    //泛型Generics
    pub(crate) generics: clean_types::Generics,

    pub full_name: String, //函数名，要来比较是否相等

    pub _trait_full_path: Option<String>, //Trait的全限定路径,因为使用trait::fun来调用函数的时候，需要将trait的全路径引入
    pub _unsafe_tag: ApiUnsafety,
}
