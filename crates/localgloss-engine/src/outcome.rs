//! 单次按键处理结果，提交由平台直接交给当前文本框。
use crate::Frame;

pub struct Outcome {
    pub handled: bool,

    pub commit: Option<String>,

    pub frame: Frame,
}
