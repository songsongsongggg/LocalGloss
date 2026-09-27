//! 候选显示行；仅包含当前组合所需的中文和英文释义。
#[derive(Clone, Default)]
pub struct Row {
    pub text: String,

    pub gloss: String,
}
