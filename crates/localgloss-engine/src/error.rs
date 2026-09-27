//! 加载错误不携带词表内容或用户输入。
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("dictionary could not be loaded")]
    Dictionary,

    #[error("glossary could not be loaded")]
    Glossary,
}
