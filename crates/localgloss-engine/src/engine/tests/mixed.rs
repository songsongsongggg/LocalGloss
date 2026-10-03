//! 中英候选的选择、光标消耗和会话清除；仅使用虚构词表。
use super::{engine, type_pinyin};
use crate::{Key, OfflineEngine, Settings, Term};

#[test]
fn explicit_english_is_first_and_commits_exactly_once() {
    for code in ["ok", "hello", "yes", "github"] {
        let mut engine = engine();
        type_pinyin(&mut engine, code);
        assert_eq!(engine.frame().rows[0].text, code);
        assert!(engine.frame().rows[0].raw);
        assert!(
            engine
                .handle(Key::HighlightedTranslation { second: false })
                .commit
                .is_none()
        );
        assert_eq!(engine.frame().preedit, code);
        assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some(code));
        assert!(engine.frame().preedit.is_empty());
        assert!(engine.engine.history().is_empty());
    }
}

#[test]
fn chinese_stays_first_and_arbitrary_letters_can_be_selected_raw() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(engine.frame().rows[0].text, "开发");
    let digit = engine.frame().rows.iter().position(|row| row.raw).unwrap() + 1;
    assert_eq!(
        engine.handle(Key::Digit(digit)).commit.as_deref(),
        Some("kaifa")
    );
    assert!(engine.frame().preedit.is_empty());
    type_pinyin(&mut engine, "foobar");
    assert!(
        engine
            .frame()
            .rows
            .iter()
            .any(|row| row.raw && row.text == "foobar")
    );
    engine.discard();
    assert!(engine.frame().rows.is_empty());
}

#[test]
fn raw_candidate_consumes_only_the_scope_before_cursor() {
    let mut engine = engine();
    type_pinyin(&mut engine, "okkaifa");
    for _ in 0..5 {
        engine.handle(Key::Left);
    }
    assert_eq!(engine.frame().rows[0].text, "ok");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("ok"));
    assert_eq!(engine.frame().preedit, "kaifa");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("开发"));
}

#[test]
fn manual_term_remains_first_even_for_explicit_english_code() {
    let mut engine = engine();
    engine
        .apply_settings(Settings {
            terms: vec![Term {
                code: "ok".into(),
                text: "确认".into(),
                gloss: "confirmed".into(),
            }],
            ..Settings::default()
        })
        .unwrap();
    type_pinyin(&mut engine, "ok");
    assert_eq!(engine.frame().rows[0].text, "确认");
    assert!(
        engine
            .frame()
            .rows
            .iter()
            .any(|row| row.raw && row.text == "ok")
    );
}

#[test]
fn abbreviated_phrase_commits_all_matched_letters() {
    let mut engine = OfflineEngine::from_tsv(
        "都是\tdou shi\t20000\n斗士\tdou shi\t500\n",
        "都是\tall are\n斗士\twarrior\n",
    )
    .unwrap();
    for code in ["dous", "doushi"] {
        type_pinyin(&mut engine, code);
        assert_eq!(engine.frame().rows[0].text, "都是");
        assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("都是"));
        assert!(engine.frame().preedit.is_empty());
        type_pinyin(&mut engine, code);
        assert_eq!(
            engine
                .handle(Key::HighlightedTranslation { second: false })
                .commit
                .as_deref(),
            Some("all are")
        );
        assert!(engine.frame().preedit.is_empty());
    }
}
