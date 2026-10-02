//! 候选显示行；仅包含当前组合所需的中文和英文释义。
#[derive(Clone, Default)]
pub struct Row {
    pub text: String,

    pub gloss: String,

    /// Tab 实际提交的第一条完整释义，显示截断不得改写它。
    pub primary_gloss: String,
}
