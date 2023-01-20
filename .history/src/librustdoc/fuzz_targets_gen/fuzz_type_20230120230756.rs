/// 代表可以通过字节序列转化的过程fuzz的类型
#[allow(dead_code)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum FuzzType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    Usize,
    Isize,
    Char,
    Bool,
    Str,
    Slice(Box<FuzzType>),
    Tuple(Vec<Box<FuzzType>>),
}

impl FuzzType {
    //是否大小固定
    pub fn is_fixed_size(&self) -> bool {
        use FuzzType::*;
        match self {
            Str | Slice(_) => false,
            Tuple(inners) => inners.iter().all(|x| x.is_fixed_size()),
            _ => true,
        }
    }

    pub fn min_size(&self) -> usize {
        use FuzzType::*;
        match self {
            I8 | U8 | Bool => 1,
            I16 | U16 => 2,
            I32 | U32 | F32 | Char => 4,
            I64 | U64 | F64 => 8,
            I128 | U128 => 16,
            Usize | Isize => std::mem::size_of::<usize>(), // 暂时当成64bit系统
            _ => 0,
            Slice(inner_fuzzable) => inner_fuzzable.min_size(),
            Str => 1,
            Tuple(inners) => {
                let mut total_length = 0;
                for inner in inners {
                    total_length = total_length + inner.min_size();
                }
                total_length
            }
        }
    }

    pub fn fixed_part_size(&self) -> usize {
        if self.is_fixed_length() {
            return self.min_length();
        } else {
            match self {
                FuzzableType::RefStr => 0,
                FuzzableType::RefSlice(..) => 0,
                FuzzableType::Tuple(inner_fuzzables) => {
                    let mut fixed_part = 0;
                    for inner_fuzzable in inner_fuzzables {
                        let inner_length = inner_fuzzable._fixed_part_length();
                        fixed_part = fixed_part + inner_length;
                    }
                    return fixed_part;
                }
                _ => self._min_length(),
            }
        }
    }
}
