//! 用公开的固定拼音样例回放首屏排序；不读取个人输入或设置。
use localgloss_engine::{Key, OfflineEngine};

fn main() {
    let resources = std::env::args_os().nth(1).expect("resource directory");
    let resources = std::path::Path::new(&resources);
    let mut engine = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    )
    .expect("dictionaries must load");
    let verify = !std::env::args().any(|arg| arg == "--report-only");
    for (pinyin, expected) in [
        ("jix", "继续"),
        ("jixu", "继续"),
        ("jx", "进行"),
        ("shiji", "世纪"),
        ("yiyi", "意义"),
        ("xian", "先"),
        ("gongzuo", "工作"),
        ("gz", "工作"),
        ("shuru", "输入"),
        ("sr", "虽然"),
        ("kaifa", "开发"),
        ("kf", "开发"),
        ("kafei", "咖啡"),
        ("nihao", "你好"),
        ("xiexie", "谢谢"),
        ("women", "我们"),
        ("zhongguo", "中国"),
        ("zhongwen", "中文"),
        ("yinsi", "隐私"),
        ("lvse", "绿色"),
        ("ceshi", "测试"),
        ("shezhi", "设置"),
        ("xianzai", "现在"),
        ("keyi", "可以"),
        ("wenti", "问题"),
        ("shiyong", "使用"),
        ("xuyao", "需要"),
        ("youhua", "优化"),
        ("ruanjian", "软件"),
        ("shijian", "时间"),
    ] {
        engine.discard();
        let start = std::time::Instant::now();
        for letter in pinyin.chars() {
            engine.handle(Key::Letter(letter));
        }
        let frame = engine.frame();
        let words: Vec<_> = frame.rows.iter().map(|row| row.text.as_str()).collect();
        if verify {
            assert!(
                words.first().is_some_and(|word| *word == expected || (pinyin == "shiji" && *word == "实际")),
                "first candidate: {pinyin}"
            );
            if pinyin == "jx" {
                assert!(words[..3].contains(&"继续"));
            }
            if pinyin == "sr" {
                assert!(words.contains(&"输入"));
            }
        }
        println!(
            "{pinyin}\t{}\t{}",
            words.join(" / "),
            start.elapsed().as_micros()
        );
    }
}
