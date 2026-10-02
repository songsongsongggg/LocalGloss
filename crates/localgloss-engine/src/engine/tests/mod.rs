//! 用虚构词表验证译词、按键和隐私边界。
#![cfg(test)]
use crate::{Key, OfflineEngine};

const DICTIONARY: &str = "开发\tkai fa\t9000\n开发者\tkai fa zhe\t3000\n开放\tkai fang\t8000\n开\tkai\t5000\n咖啡\tka fei\t4000\n";
const GLOSSARY: &str =
    "开发\tv. develop\tv. exploit\n开发者\tn. developer\n开放\tv. open\n开\tv. open\n";

fn engine() -> OfflineEngine {
    OfflineEngine::from_tsv(DICTIONARY, GLOSSARY).unwrap()
}

fn type_pinyin(engine: &mut OfflineEngine, text: &str) {
    for letter in text.chars() {
        assert!(engine.handle(Key::Letter(letter)).handled);
    }
}

#[test]
fn shows_offline_gloss_and_commits_chinese() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(engine.frame().rows[0].text, "开发");
    assert_eq!(engine.frame().rows[0].gloss, "develop · exploit");
    assert_eq!(engine.frame().rows[0].primary_gloss, "develop");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("开发"));
    assert!(engine.frame().preedit.is_empty());
}

#[test]
fn translation_commits_without_exposing_a_clipboard_api() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine
            .handle(Key::Translation {
                digit: 1,
                second: false
            })
            .commit
            .as_deref(),
        Some("develop")
    );
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine
            .handle(Key::Translation {
                digit: 1,
                second: true
            })
            .commit
            .as_deref(),
        Some("exploit")
    );
}

#[test]
fn primary_display_matches_full_commit_without_splitting_literal_separators() {
    let first = "first · literal separator; 完整译词 ".repeat(12);
    let mut engine =
        OfflineEngine::from_tsv(DICTIONARY, &format!("开发\t{first}\tsecond sense\n")).unwrap();
    type_pinyin(&mut engine, "kaifa");
    let displayed = engine.frame().rows[0].primary_gloss.clone();
    assert_eq!(displayed, first.trim());
    assert!(engine.frame().rows[0].gloss.contains("second sense"));
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: false })
            .commit,
        Some(displayed)
    );
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: true })
            .commit
            .as_deref(),
        Some("second sense")
    );
}

#[test]
fn missing_gloss_does_not_consume_input() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kafei");
    assert_eq!(engine.frame().rows[0].text, "咖啡");
    assert!(engine.frame().rows[0].gloss.is_empty());
    assert!(
        engine
            .handle(Key::Translation {
                digit: 1,
                second: false
            })
            .commit
            .is_none()
    );
    assert_eq!(engine.frame().preedit, "kafei");
}

#[test]
fn shorter_candidate_keeps_only_remaining_pinyin() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifazhe");
    let digit = engine
        .frame()
        .rows
        .iter()
        .position(|row| row.text == "开发")
        .unwrap()
        + 1;
    assert_eq!(
        engine.handle(Key::Digit(digit)).commit.as_deref(),
        Some("开发")
    );
    assert_eq!(engine.frame().preedit, "zhe");
    assert!(engine.engine.history().is_empty());
}

#[test]
fn security_transition_discards_then_passes_every_key_through() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    engine.set_blocked(true);
    for key in [
        Key::Letter('a'),
        Key::Space,
        Key::Enter,
        Key::Translation {
            digit: 1,
            second: false,
        },
    ] {
        let outcome = engine.handle(key);
        assert!(!outcome.handled);
        assert!(outcome.commit.is_none());
        assert!(outcome.frame.rows.is_empty());
        assert!(outcome.frame.preedit.is_empty());
    }
    engine.set_blocked(false);
    assert!(engine.finish_raw().is_none());
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("开发"));
}

#[test]
fn focus_boundary_removes_old_input_and_candidates() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    engine.discard();
    assert!(engine.handle(Key::Space).commit.is_none());
    assert!(engine.frame().preedit.is_empty());
    assert!(engine.frame().rows.is_empty());
    assert!(engine.engine.history().is_empty());
}

#[test]
fn escape_backspace_raw_and_unknown_keys_are_bounded() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    engine.handle(Key::Backspace);
    assert_eq!(engine.frame().preedit, "kaif");
    assert_eq!(engine.handle(Key::Enter).commit.as_deref(), Some("kaif"));
    type_pinyin(&mut engine, "kaifa");
    let result = engine.handle(Key::PassThrough);
    assert!(!result.handled);
    assert_eq!(result.commit.as_deref(), Some("kaifa"));
    type_pinyin(&mut engine, "kaifa");
    engine.handle(Key::Escape);
    assert!(engine.finish_raw().is_none());
    assert!(!engine.handle(Key::Backspace).handled);
}

#[test]
fn committed_input_is_not_retained_in_core_history() {
    let mut engine = engine();
    for _ in 0..4 {
        type_pinyin(&mut engine, "kaifa");
        engine.handle(Key::Space);
        assert!(engine.engine.history().is_empty());
        assert!(!engine.engine.prediction_enabled());
        assert!(engine.engine.is_private());
    }
}

#[test]
fn input_limit_flushes_raw_without_swallowing_next_letter() {
    let mut engine = engine();
    for _ in 0..64 {
        assert!(engine.handle(Key::Letter('a')).handled);
    }
    let result = engine.handle(Key::Letter('a'));
    assert!(!result.handled);
    assert_eq!(result.commit.unwrap().len(), 64);
    assert!(engine.frame().preedit.is_empty());
}

mod navigation;

mod preferences;
