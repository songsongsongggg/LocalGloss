//! 用户主动配置的选项；不含输入历史、账户或网络配置。
use serde::{Deserialize, Serialize};

use crate::Term;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub version: u8,

    pub font_size: u8,

    pub page_size: usize,

    pub tab_translation: bool,

    pub symbol_paging: bool,

    pub chinese_punctuation: bool,

    pub terms: Vec<Term>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            font_size: 16,
            page_size: 9,
            tab_translation: true,
            symbol_paging: true,
            chinese_punctuation: false,
            terms: Vec::new(),
        }
    }
}

impl Settings {
    pub const MAX_BYTES: usize = 256 * 1024;

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > Self::MAX_BYTES {
            return Err("设置文件过大");
        }
        let settings: Self = serde_json::from_slice(bytes).map_err(|_| "设置格式不正确")?;
        settings.validate()?;
        Ok(settings)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.version != 1 {
            return Err("不支持的设置版本");
        }
        if !(14..=22).contains(&self.font_size) || !(3..=9).contains(&self.page_size) {
            return Err("字号须为 14–22，候选数须为 3–9");
        }
        if self.terms.len() > 500 {
            return Err("最多保存 500 个主动添加的词条");
        }
        let mut occupied = std::collections::HashSet::new();
        for term in &self.terms {
            if term.code.is_empty()
                || term.code.len() > 32
                || !term.code.bytes().all(|c| c.is_ascii_lowercase())
            {
                return Err("输入码须为 1–32 个小写字母");
            }
            if term.text.contains('|')
                || term.text.trim().is_empty()
                || term.text.chars().count() > 64
                || term.text.chars().any(char::is_control)
                || term.gloss.chars().count() > 240
                || term.gloss.chars().any(char::is_control)
            {
                return Err("词条须为单行，中文最多 64 字，译词最多 240 字");
            }
            if !occupied.insert(&term.code) {
                return Err("同一输入码只允许一个置顶词条");
            }
        }
        Ok(())
    }
}
