//! 设置、手动词条、混输、详情的行为与隐私边界。
use super::{engine, type_pinyin};
use crate::{Key, Settings, Term};

#[test]
fn invalid_settings_never_change_existing_preferences_or_composition() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    let settings = Settings {
        page_size: 0,
        ..Settings::default()
    };
    assert!(engine.apply_settings(settings).is_err());
    assert_eq!(engine.frame().preedit, "kaifa");
    assert_eq!(engine.settings().page_size, 9);
    assert!(Settings::parse(br#"{"version":2}"#).is_err());
    assert!(Settings::parse(br#"{"network":true}"#).is_err());
    assert!(Settings::parse(&vec![b' '; Settings::MAX_BYTES + 1]).is_err());
}

#[test]
fn manual_term_is_pinned_only_for_its_code_and_translates_without_history() {
    let mut engine = engine();
    let settings = Settings {
        terms: vec![Term {
            code: "kaifa".into(),
            text: "开发工作".into(),
            gloss: "development work".into(),
        }],
        ..Settings::default()
    };
    engine.apply_settings(settings).unwrap();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(engine.frame().rows[0].text, "开发工作");
    assert_eq!(
        engine
            .handle(Key::HighlightedTranslation { second: false })
            .commit
            .as_deref(),
        Some("development work")
    );
    assert!(!engine.is_composing());
    assert!(engine.engine.history().is_empty());
    type_pinyin(&mut engine, "kai");
    assert!(engine.frame().rows.iter().all(|row| row.text != "开发工作"));
    engine.apply_settings(Settings::default()).unwrap();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(engine.frame().rows[0].text, "开发");
}

#[test]
fn duplicate_codes_and_multiline_terms_are_rejected() {
    let term = Term {
        code: "kf".into(),
        text: "开发".into(),
        gloss: "develop".into(),
    };
    let mut settings = Settings {
        terms: vec![term.clone(), term],
        ..Settings::default()
    };
    assert!(settings.validate().is_err());
    settings.terms.pop();
    settings.terms[0].text = "a\nb".into();
    assert!(settings.validate().is_err());
}

#[test]
fn explicit_english_and_url_tokens_do_not_convert_later_letters() {
    let mut engine = engine();
    type_pinyin(&mut engine, "http");
    let colon = engine.handle(Key::Literal(':'));
    assert_eq!(colon.commit.as_deref(), Some("http"));
    assert!(!colon.handled);
    assert!(!engine.handle(Key::Letter('a')).handled);
    assert!(!engine.handle(Key::Digit(1)).handled);
    assert!(!engine.handle(Key::Space).handled);
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine.handle(Key::ToggleEnglish).commit.as_deref(),
        Some("kaifa")
    );
    assert!(engine.english_mode());
    assert!(!engine.handle(Key::Letter('a')).handled);
    engine.handle(Key::ToggleEnglish);
    assert!(!engine.english_mode());
    type_pinyin(&mut engine, "kaifa");
    engine.set_blocked(true);
    assert!(!engine.handle(Key::ToggleEnglish).handled);
    assert!(!engine.handle(Key::Literal('.')).handled);
    engine.set_blocked(false);
    assert!(!engine.is_composing());
}

#[test]
fn punctuation_is_opt_in_and_does_not_drop_remaining_input() {
    let mut engine = engine();
    type_pinyin(&mut engine, "kaifa");
    let literal = engine.handle(Key::Literal(','));
    assert!(!literal.handled);
    assert_eq!(literal.commit.as_deref(), Some("kaifa"));
    engine
        .apply_settings(Settings {
            chinese_punctuation: true,
            ..Settings::default()
        })
        .unwrap();
    type_pinyin(&mut engine, "kaifa");
    assert_eq!(
        engine.handle(Key::Literal(',')).commit.as_deref(),
        Some("开发，")
    );
    assert_eq!(
        engine.handle(Key::Literal('.')).commit.as_deref(),
        Some("。")
    );
    assert!(engine.engine.history().is_empty());
}

#[test]
fn detail_pages_cover_full_translation_and_escape_preserves_composition() {
    let mut engine = engine();
    let gloss = "完整释义".repeat(60);
    engine
        .apply_settings(Settings {
            page_size: 3,
            terms: vec![Term {
                code: "kf".into(),
                text: "开发".into(),
                gloss: gloss.clone(),
            }],
            ..Settings::default()
        })
        .unwrap();
    type_pinyin(&mut engine, "kf");
    engine.handle(Key::ToggleDetails);
    assert_eq!(engine.frame().pages, 2);
    let first = engine.frame().detail.unwrap();
    let second = engine.handle(Key::PageNext).frame.detail.unwrap();
    assert_eq!(first + &second, format!("开发\n{gloss}"));
    engine.handle(Key::Escape);
    assert!(engine.frame().detail.is_none());
    assert_eq!(engine.frame().preedit, "kf");
    engine.discard();
    assert!(engine.frame().detail.is_none());
}
