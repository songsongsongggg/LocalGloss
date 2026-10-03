//! 原样字母候选留在首屏；明确英文词优先，其他输入保持中文优先。
use qingjian_core::{Candidate, CandidateKind};

use crate::OfflineEngine;

const ENGLISH_FIRST: &[&str] = &[
    "ok", "hello", "hi", "yes", "thanks", "bye", "github", "chatgpt", "email", "http", "https",
];

impl OfflineEngine {
    pub(super) fn insert_raw_candidate(&mut self) {
        let raw = self.engine.composition().scope();
        if raw.is_empty() || !raw.bytes().all(|byte| byte.is_ascii_lowercase()) {
            return;
        }
        // 主动词条的优先位置不被内置英文优先规则覆盖。
        let manual = self.settings.terms.iter().any(|term| term.code == raw);
        let position = if !manual && ENGLISH_FIRST.contains(&raw) {
            0
        } else {
            self.settings
                .page_size
                .saturating_sub(1)
                .min(self.candidates.len())
        };
        self.candidates.insert(
            position,
            Candidate {
                text: raw.to_owned(),
                kind: CandidateKind::English,
                syllables: Vec::new(),
                reading: None,
                translation: None,
                aux_code: None,
            },
        );
        self.candidates.truncate(super::MAX_CANDIDATES);
    }
}
