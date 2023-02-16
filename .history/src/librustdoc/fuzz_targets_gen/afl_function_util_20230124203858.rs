//use crate::fuzz_targets_gen::afl_param_util;

use super::afl_param_util::AflForParams;
use super::fuzz_type::FuzzType;

/// 生成代测试文件
#[allow(dead_code)]
struct AflFunctionHelper {
    fuzz_params: Vec<FuzzType>,
}

impl AflFunctionHelper {
    pub fn generate_afl_file(&self, test_index: usize) -> String {
        let mut res = self._to_afl_except_main(_api_graph, test_index);
        res.push_str(self.generate_afl_main(test_index).as_str());
        res
    }

    pub fn generate_afl_help(&self, _api_graph: &ApiGraph, test_index: usize) -> String {
        let mut res = String::new();
        //加入可能需要开启的feature gate
        let feature_gates = afl_util::_get_feature_gates_of_sequence(&self.fuzzable_params);

        if feature_gates.is_some() {
            for feature_gate in &feature_gates.unwrap() {
                let feature_gate_line = format!("{feature_gate}\n", feature_gate = feature_gate);
                res.push_str(feature_gate_line.as_str());
            }
        }

        res.push_str("#[macro_use]\n");
        res.push_str("extern crate afl;\n");
        res.push_str(format!("extern crate {};\n", _api_graph._crate_name).as_str());

        let prelude_helper_functions = self._prelude_helper_functions();
        if let Some(prelude_functions) = prelude_helper_functions {
            res.push_str(prelude_functions.as_str());
        }

        let afl_helper_functions = self._afl_helper_functions();
        if let Some(afl_functions) = afl_helper_functions {
            res.push_str(afl_functions.as_str());
        }
        res.push_str(self._to_well_written_function(_api_graph, test_index, 0).as_str());
        res.push('\n');
        res
    }

    /// afl的main函数
    /// fn main() {
    ///     fuzz!(|data: &[u8]|{
    ///         ...
    ///     });
    /// }
    #[allow(dead_code)]
    pub fn generate_afl_main(&self, test_index: usize) -> String {
        let mut res = String::new();
        let indent = generate_indent(1);
        res.push_str("fn main() {\n");
        res.push_str(indent.as_str());
        res.push_str("fuzz!(|data: &[u8]| {\n");
        res.push_str(self.generate_afl_closure(1, test_index).as_str());
        res.push_str(indent.as_str());
        res.push_str("});\n");
        res.push_str("}\n");
        res
    }

    /// 闭包中的函数体，主要用来初始化参数和调用test_function_i `fuzz!(|data: &[u8]|{...});`
    //#[allow(dead_code)]
    fn generate_afl_closure(&self, outer_tab: usize, test_index: usize) -> String {
        let mut res = String::new();
        let indent = generate_indent(outer_tab + 1);

        res.push_str(format!("{indent}//actual body emit\n", indent = indent).as_str());

        // 获取test_function所有参数的最小的长度
        let min_len = self.fuzz_params_min_length();
        res.push_str(
            format!(
                "{indent}if data.len() < {min_len} {{return;}}\n",
                indent = indent,
                min_len = min_len
            )
            .as_str(),
        );

        //固定部分结束的地方就是动态部分开始的地方
        let dynamic_param_start_index = self.fuzz_params_fixed_size_part_length();
        let fuzz_params_dynamic_size_parts_number = self.fuzz_params_dynamic_size_parts_number();
        let dynamic_length_name = "dynamic_length";

        // let dynamic_length = (data.len() - dynamic_param_param_start_index) / dynamic_param_number
        // 计算每个dynamic part的长度！！！
        let every_dynamic_length = format!(
            "let {dynamic_length_name} = (data.len() - {dynamic_param_start_index}) / {fuzz_params_dynamic_size_parts_number}",
            dynamic_length_name = dynamic_length_name,
            dynamic_param_start_index = dynamic_param_start_index,
            fuzz_params_dynamic_size_parts_number = fuzz_params_dynamic_size_parts_number
        );

        if !self.is_all_fuzz_params_fixed_size() {
            res.push_str(
                format!(
                    "{indent}{every_dynamic_length};\n",
                    indent = indent,
                    every_dynamic_length = every_dynamic_length
                )
                .as_str(),
            );
        }

        //每次迭代固定长度开始的位置
        let mut fixed_start_index = 0;
        //当前这是第几个动态长度的变量
        let mut dynamic_index = 0;

        for (i, param_type) in self.fuzz_params.iter().enumerate() {
            let param_init_statement = param_type.generate_param_initial_statement(
                i,
                fixed_start_index,
                dynamic_param_start_index,
                dynamic_index,
                fuzz_params_dynamic_size_parts_number,
                &dynamic_length_name.to_string(),
            );
            res.push_str(
                format!(
                    "{indent}{param_init_statement}\n",
                    indent = indent,
                    param_init_statement = param_init_statement
                )
                .as_str(),
            );
            fixed_start_index += param_type.fixed_size_part_size();
            dynamic_index += param_type.dynamic_size_parts_number();
        }

        //参数初始化完成，可以调用test_function
        let mut test_function_call =
            format!("{indent}test_function{test_index}(", indent = indent, test_index = test_index);
        for i in 0..self.fuzz_params.len() {
            if i != 0 {
                test_function_call.push_str(" ,");
            }
            test_function_call.push_str(format!("_param{}", i).as_str());
        }
        test_function_call.push_str(");\n");
        res.push_str(test_function_call.as_str());

        res
    }

    fn fuzz_params_min_length(&self) -> usize {
        let mut min_length = 0;
        for param_type in &self.fuzz_params {
            min_length += param_type.min_size();
        }
        min_length
    }

    fn fuzz_params_fixed_size_part_length(&self) -> usize {
        let mut fixed_size_part_length = 0;
        for param_type in &self.fuzz_params {
            fixed_size_part_length += param_type.fixed_size_part_size();
        }
        fixed_size_part_length
    }

    fn fuzz_params_dynamic_size_parts_number(&self) -> usize {
        let mut num = 0;
        for param_type in &self.fuzz_params {
            num += param_type.dynamic_size_parts_number();
        }
        num
    }

    fn is_all_fuzz_params_fixed_size(&self) -> bool {
        self.fuzz_params.iter().all(|x| x.is_fixed_size())
    }
}

/// 生成每行代码前面的空格
fn generate_indent(tab_num: usize) -> String {
    let mut indent = String::new();
    for _ in 0..tab_num {
        indent.push_str("    ");
    }
    indent
}
