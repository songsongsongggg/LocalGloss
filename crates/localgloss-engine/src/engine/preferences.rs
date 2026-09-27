//! 设置应用、主动词条释义与混输；不读取输入框正文或持久化组合。
use qingjian_core::{CustomPhrase, Language, Sense, Translation};

use crate::{OfflineEngine, Outcome, Settings};

impl OfflineEngine {
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn english_mode(&self) -> bool {
        self.english
    }

    pub fn apply_settings(&mut self, settings: Settings) -> Result<(), &'static str> {
        settings.validate()?;
        let phrases = settings
            .terms
            .iter()
            .map(|term| CustomPhrase {
                code: term.code.clone(),
                text: term.text.clone(),
                position: 1,
                enabled: true,
            })
            .collect();
        self.engine
            .set_custom_phrases(phrases)
            .map_err(|_| "词条无法应用")?;
        self.discard();
        self.settings = settings;
        Ok(())
    }

    pub(super) fn annotate_manual_terms(&mut self) {
        let raw = self.engine.raw_preedit().text;
        if let Some(term) = self.settings.terms.iter().find(|term| term.code == raw) {
            for candidate in &mut self.candidates {
                if candidate.text == term.text && !term.gloss.is_empty() {
                    candidate.translation = Some(Translation::new(
                        Language::English,
                        vec![Sense {
                            part_of_speech: None,
                            text: term.gloss.clone(),
                            reading: None,
                            fresh: false,
                        }],
                    ));
                }
            }
        }
    }

    pub(super) fn literal(&mut self, character: char) -> Outcome {
        // URL、邮箱、标识符明确进入原样 token；遇空格/回车/快捷键或会话清除才结束。
        if matches!(character, ':' | '/' | '@' | '_' | '\\') {
            let text = self.finish_raw();
            self.raw_token = true;
            return self.outcome(false, text);
        }
        let punctuation = if self.settings.chinese_punctuation {
            match character {
                ',' => Some("，"),
                '.' => Some("。"),
                '?' => Some("？"),
                '!' => Some("！"),
                ';' => Some("；"),
                _ => None,
            }
        } else {
            None
        };
        if let Some(punctuation) = punctuation {
            let mut text = self.select(self.highlighted, None).unwrap_or_default();
            if let Some(remaining) = self.finish_raw() {
                text.push_str(&remaining);
            }
            text.push_str(punctuation);
            return self.outcome(true, Some(text));
        }
        let text = self.finish_raw();
        self.outcome(false, text)
    }
}
