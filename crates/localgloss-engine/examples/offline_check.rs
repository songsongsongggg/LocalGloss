//! 仅使用虚构测试拼音，检查完整随包词表在禁止联网和写文件的环境中可用。
use localgloss_engine::{Key, OfflineEngine};

fn main() {
    let resources = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/cedict/generated")
        });
    let started = std::time::Instant::now();
    let mut engine = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    )
    .expect("bundled dictionaries must load");
    let loaded = started.elapsed();
    for (pinyin, expected) in [
        ("kaifa", "开发"),
        ("kafei", "咖啡"),
        ("nihao", "你好"),
        ("xiexie", "谢谢"),
        ("zhongguo", "中国"),
        ("zhongwen", "中文"),
        ("gongzuo", "工作"),
        ("women", "我们"),
        ("yinsi", "隐私"),
        ("lvse", "绿色"),
    ] {
        for letter in pinyin.chars() {
            engine.handle(Key::Letter(letter));
        }
        let frame = engine.frame();
        let digit = frame
            .rows
            .iter()
            .position(|row| row.text == expected)
            .unwrap_or_else(|| panic!("fixture candidate missing: {pinyin}"))
            + 1;
        let result = engine.handle(Key::Translation {
            digit,
            second: false,
        });
        assert!(
            result
                .commit
                .as_deref()
                .is_some_and(|text| !text.is_empty())
        );
        assert!(result.frame.preedit.is_empty());
    }
    for letter in "kafei".chars() {
        engine.handle(Key::Letter(letter));
    }
    engine.set_blocked(true);
    assert!(!engine.handle(Key::Letter('a')).handled);
    assert!(engine.frame().preedit.is_empty());
    engine.set_blocked(false);
    assert!(engine.finish_raw().is_none());
    println!(
        "bundled offline flow passed; load={}ms; flow={}ms",
        loaded.as_millis(),
        started.elapsed().as_millis()
    );
}
