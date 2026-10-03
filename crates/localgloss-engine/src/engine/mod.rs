//! 固定装配只读词表；敏感状态和焦点边界不保留输入。
mod mixed;
mod preferences;
mod tests;

use std::path::Path;

use qingjian_core::{Candidate, Engine, Language};
use qingjian_dictionary::Dictionary;
use qingjian_translate::Glossary;

use crate::{Frame, Key, LoadError, Outcome, Row, Settings};

const MAX_CANDIDATES: usize = 90;
const MAX_INPUT_BYTES: usize = 64;

pub struct OfflineEngine {
    engine: Engine,

    candidates: Vec<Candidate>,

    highlighted: usize,

    blocked: bool,

    settings: Settings,

    english: bool,

    raw_token: bool,

    details: bool,

    detail_page: usize,
}

impl OfflineEngine {
    pub fn from_paths(dictionary: &Path, glossary: &Path) -> Result<Self, LoadError> {
        let dictionary = Dictionary::from_path(dictionary).map_err(|_| LoadError::Dictionary)?;
        let glossary =
            Glossary::from_path(Language::English, glossary).map_err(|_| LoadError::Glossary)?;
        Ok(Self::new(dictionary, glossary))
    }

    pub fn from_tsv(dictionary: &str, glossary: &str) -> Result<Self, LoadError> {
        let dictionary = Dictionary::parse(dictionary).map_err(|_| LoadError::Dictionary)?;
        let glossary =
            Glossary::parse(Language::English, glossary).map_err(|_| LoadError::Glossary)?;
        Ok(Self::new(dictionary, glossary))
    }

    fn new(dictionary: Dictionary, glossary: Glossary) -> Self {
        // Engine 的缺省服务均不落盘；不使用 PersonalGlossary 或任何用户数据服务。
        let mut engine = Engine::new(dictionary).with_translator(Box::new(glossary));
        engine.set_learning(false);
        engine.set_private(true);
        Self {
            engine,
            candidates: Vec::new(),
            highlighted: 0,
            blocked: false,
            settings: Settings::default(),
            english: false,
            raw_token: false,
            details: false,
            detail_page: 0,
        }
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        if self.blocked != blocked || blocked {
            self.discard();
        }
        self.blocked = blocked;
    }

    pub fn discard(&mut self) {
        self.engine.discard_input();
        self.candidates.clear();
        self.highlighted = 0;
        self.details = false;
        self.detail_page = 0;
        self.raw_token = false;
    }

    pub fn is_composing(&self) -> bool {
        !self.engine.composition().is_empty()
    }

    pub fn finish_raw(&mut self) -> Option<String> {
        if self.blocked || !self.is_composing() {
            self.discard();
            return None;
        }
        let text = self.engine.raw_preedit().text;
        self.discard();
        Some(text)
    }

    pub fn frame(&self) -> Frame {
        if self.blocked || !self.is_composing() {
            return Frame::default();
        }
        let raw = self.engine.raw_preedit();
        let page = self.highlighted / self.settings.page_size;
        let rows: Vec<Row> = self
            .candidates
            .iter()
            .skip(page * self.settings.page_size)
            .take(self.settings.page_size)
            .map(|candidate| Row {
                text: candidate.text.clone(),
                raw: candidate.kind == qingjian_core::CandidateKind::English,
                primary_gloss: candidate
                    .translation
                    .as_ref()
                    .and_then(|translation| translation.senses().first())
                    .map(|sense| sense.text.clone())
                    .unwrap_or_default(),
                gloss: candidate
                    .translation
                    .as_ref()
                    .map(|translation| {
                        translation
                            .senses()
                            .iter()
                            .map(|sense| sense.text.as_str())
                            .collect::<Vec<_>>()
                            .join(" · ")
                    })
                    .unwrap_or_default(),
            })
            .collect();
        let detail = self.details.then(|| {
            rows.get(self.highlighted % self.settings.page_size)
                .map(|row| {
                    format!(
                        "{}\n{}",
                        row.text,
                        if row.gloss.is_empty() {
                            "暂无译词"
                        } else {
                            &row.gloss
                        }
                    )
                })
                .unwrap_or_else(|| "暂无候选".into())
        });
        let detail_pages = detail
            .as_ref()
            .map_or(1, |text| text.chars().count().div_ceil(240).max(1));
        let detail = detail.map(|text| {
            text.chars()
                .skip(self.detail_page * 240)
                .take(240)
                .collect()
        });
        Frame {
            detail,
            english: self.english,
            cursor: raw.cursor_bytes,
            preedit: raw.text,
            rows,
            highlighted: self.highlighted % self.settings.page_size,
            page: if self.details { self.detail_page } else { page },
            pages: if self.details {
                detail_pages
            } else {
                self.candidates
                    .len()
                    .div_ceil(self.settings.page_size)
                    .max(1)
            },
        }
    }

    pub fn handle(&mut self, key: Key) -> Outcome {
        if self.blocked {
            self.discard();
            return self.outcome(false, None);
        }
        if matches!(key, Key::ToggleEnglish) {
            let text = self.finish_raw();
            self.english = !self.english;
            return self.outcome(true, text);
        }
        if self.english {
            return self.outcome(false, None);
        }
        if self.raw_token {
            if matches!(
                key,
                Key::Space | Key::Enter | Key::Escape | Key::PassThrough
            ) {
                self.raw_token = false;
            }
            return self.outcome(false, None);
        }
        if let Key::Literal(character) = key {
            return self.literal(character);
        }
        if let Key::Letter(letter) = key {
            if !(letter.is_ascii_lowercase() || (letter == '\'' && self.is_composing())) {
                let text = self.finish_raw();
                return self.outcome(false, text);
            }
            if self.engine.composition().text().len() >= MAX_INPUT_BYTES {
                let text = self.finish_raw();
                return self.outcome(false, text);
            }
            self.engine.push(letter);
            self.refresh();
            return self.outcome(true, None);
        }
        if !self.is_composing() {
            return self.outcome(false, None);
        }
        if self.details {
            match key {
                Key::PageNext => {
                    self.detail_page = (self.detail_page + 1).min(self.frame().pages - 1);
                    return self.outcome(true, None);
                }
                Key::PagePrevious => {
                    self.detail_page = self.detail_page.saturating_sub(1);
                    return self.outcome(true, None);
                }
                Key::Escape => {
                    self.details = false;
                    self.detail_page = 0;
                    return self.outcome(true, None);
                }
                Key::Next | Key::Previous => self.detail_page = 0,
                _ => {}
            }
        }
        match key {
            Key::Backspace => {
                self.engine.backspace();
                self.refresh();
            }
            Key::Escape => self.discard(),
            Key::Enter => {
                let text = self.finish_raw();
                return self.outcome(true, text);
            }
            Key::Space => {
                let text = self
                    .select(self.highlighted, None)
                    .or_else(|| self.finish_raw());
                return self.outcome(true, text);
            }
            Key::Digit(digit) => {
                if let Some(index) = self.page_index(digit) {
                    let text = self.select(index, None);
                    return self.outcome(true, text);
                }
                let text = self.finish_raw();
                return self.outcome(false, text);
            }
            Key::Translation { digit, second } => {
                let text = self
                    .page_index(digit)
                    .and_then(|index| self.select(index, Some(usize::from(second))));
                return self.outcome(true, text);
            }
            Key::HighlightedTranslation { second } => {
                let text = self.select(self.highlighted, Some(usize::from(second)));
                return self.outcome(true, text);
            }
            Key::Previous => self.highlighted = self.highlighted.saturating_sub(1),
            Key::Next => {
                self.highlighted =
                    (self.highlighted + 1).min(self.candidates.len().saturating_sub(1))
            }
            Key::PagePrevious => {
                let page = (self.highlighted / self.settings.page_size).saturating_sub(1);
                self.highlighted = page * self.settings.page_size;
            }
            Key::PageNext => {
                let last_page = self.candidates.len().saturating_sub(1) / self.settings.page_size;
                let page = (self.highlighted / self.settings.page_size + 1).min(last_page);
                self.highlighted = page * self.settings.page_size;
            }
            Key::Left => {
                self.engine.move_cursor_left();
                self.refresh();
            }
            Key::Right => {
                self.engine.move_cursor_right();
                self.refresh();
            }
            Key::PassThrough => {
                let text = self.finish_raw();
                return self.outcome(false, text);
            }
            Key::ToggleDetails => {
                self.details = !self.details;
                self.detail_page = 0;
            }
            Key::Letter(_) | Key::Literal(_) | Key::ToggleEnglish => unreachable!(),
        }
        self.outcome(true, None)
    }

    fn page_index(&self, digit: usize) -> Option<usize> {
        if !(1..=self.settings.page_size).contains(&digit) {
            return None;
        }
        let index =
            self.highlighted / self.settings.page_size * self.settings.page_size + digit - 1;
        (index < self.candidates.len()).then_some(index)
    }

    fn select(&mut self, index: usize, sense: Option<usize>) -> Option<String> {
        let candidate = self.candidates.get(index)?.clone();
        let text = match sense {
            Some(sense) => self.engine.commit_translation(&candidate, sense)?,
            None => self.engine.commit(&candidate),
        };
        // 上屏可留下未消耗的拼音；清除已上屏历史，避免它进入下一次查询。
        let remaining = self.engine.raw_preedit().text;
        let cursor = self.engine.raw_preedit().cursor_bytes;
        self.discard();
        if !remaining.is_empty() {
            self.engine.set_input(&remaining);
            while self.engine.composition().cursor() > cursor {
                self.engine.move_cursor_left();
            }
            self.refresh();
        }
        Some(text)
    }

    fn refresh(&mut self) {
        self.details = false;
        self.detail_page = 0;
        self.highlighted = 0;
        self.candidates.clear();
        if !self.is_composing() {
            self.discard();
            return;
        }
        if let Ok(mut query) = self.engine.query() {
            query.candidates.items.truncate(MAX_CANDIDATES);
            self.engine.annotate(&mut query.candidates);
            self.candidates = query.candidates.items;
            self.annotate_manual_terms();
        }
        self.insert_raw_candidate();
    }

    fn outcome(&self, handled: bool, commit: Option<String>) -> Outcome {
        Outcome {
            handled,
            commit,
            frame: self.frame(),
        }
    }
}
