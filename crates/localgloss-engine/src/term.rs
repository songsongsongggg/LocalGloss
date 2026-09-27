//! 仅来自用户主动填写的置顶词条，不从输入会话生成。
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub code: String,

    pub text: String,

    #[serde(default)]
    pub gloss: String,
}
