#[must_use]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SlangOptimizationLevel {
    None = 0,
    Default = 1,
    High = 2,
    Maximal = 3,
}
