//! 使用公开词表和固定测试表达验证中英混输与连续候选消耗。
use localgloss_engine::{Key, OfflineEngine};
use std::path::PathBuf;

fn main() {
    let directory = PathBuf::from(std::env::args_os().nth(1).expect("data directory required"));
    let mut engine = OfflineEngine::from_paths(
        &directory.join("dict.tsv"),
        &directory.join("glossary-en.tsv"),
    )
    .expect("dictionary must load");
    for (code, expected) in [
        ("ok", "ok"),
        ("hello", "hello"),
        ("dous", "都是"),
        ("doushi", "都是"),
        ("yes", "yes"),
        ("meiy", "没有"),
        ("haiy", "还有"),
        ("yeshi", "也是"),
        ("haode", "好的"),
        ("woxiang", "我想"),
    ] {
        for letter in code.chars() {
            engine.handle(Key::Letter(letter));
        }
        assert_eq!(
            engine.frame().rows[0].text,
            expected,
            "first candidate for {code}"
        );
        assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some(expected));
        assert!(engine.frame().preedit.is_empty());
        println!("{code} -> {expected}");
    }
    for (code, expected) in [("doushikaifa", "都是开发"), ("woxiangkaifa", "我想开发")] {
        for letter in code.chars() {
            engine.handle(Key::Letter(letter));
        }
        assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some(expected));
        assert!(engine.frame().preedit.is_empty());
        println!("{code} -> {expected}");
    }
    // Rime 收录而译词表缺失的公开组合，不应因没有英文而阻止中文上屏。
    for letter in "wode".chars() {
        engine.handle(Key::Letter(letter));
    }
    assert_eq!(engine.frame().rows[0].text, "我的");
    assert!(engine.frame().rows[0].primary_gloss.is_empty());
    assert!(
        engine
            .handle(Key::Translation {
                digit: 1,
                second: false
            })
            .commit
            .is_none()
    );
    assert_eq!(engine.frame().preedit, "wode");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("我的"));
    assert!(engine.frame().preedit.is_empty());
    println!("Rime-only candidate: 我的; missing gloss preserves Chinese input");
    for letter in "doushikaifa".chars() {
        engine.handle(Key::Letter(letter));
    }
    let digit = engine
        .frame()
        .rows
        .iter()
        .position(|row| row.text == "都是")
        .expect("prefix candidate")
        + 1;
    assert_eq!(
        engine.handle(Key::Digit(digit)).commit.as_deref(),
        Some("都是")
    );
    assert_eq!(engine.frame().preedit, "kaifa");
    assert_eq!(engine.handle(Key::Space).commit.as_deref(), Some("开发"));
    assert!(engine.frame().preedit.is_empty());
    println!("continuous composition: 都是 -> 开发; no residual letters");
}
