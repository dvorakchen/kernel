//! 一些工具类

/// 向上对齐
///
/// # Arguments
/// - number: 对齐前的数
/// - align: 对齐的字节数
pub fn align_top(number: usize, align: usize) -> usize {
    (number + align - 1) & !(align - 1)
}

/// 向下对齐
///
/// # Arguments
/// - number: 对齐前的数
/// - align: 对齐的字节数
pub fn align_bottom(number: usize, align: usize) -> usize {
    number & !(align - 1)
}

