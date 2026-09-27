//! 当前组合的界面帧，在焦点和隐私边界清除。
use crate::Row;

#[derive(Clone, Default)]
pub struct Frame {
    pub preedit: String,

    pub cursor: usize,

    pub rows: Vec<Row>,

    pub highlighted: usize,

    pub page: usize,

    pub pages: usize,

    pub detail: Option<String>,

    pub english: bool,
}
