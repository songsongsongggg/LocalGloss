//! 固定样例的排序报告与逐键计时；报告只含公开样例，不读取个人输入。
use localgloss_engine::{Key, OfflineEngine};
use serde_json::json;
use std::time::Instant;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let resources = std::path::Path::new(args.get(1).expect("resource directory"));
    let started = Instant::now();
    let mut engine = OfflineEngine::from_paths(
        &resources.join("dict.tsv"),
        &resources.join("glossary-en.tsv"),
    )
    .expect("dictionaries");
    let load_us = started.elapsed().as_micros();
    let fixtures: Vec<_> = include_str!("fixtures/daily.txt")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .collect();
    assert!(fixtures.len() >= 150);
    let mut cases = Vec::new();
    let mut latencies = Vec::new();
    // 首轮预热单列；随后三轮记录逐键耗时，避免把整串耗时误报为逐键延迟。
    for round in 0..4 {
        for fields in &fixtures {
            assert_eq!(fields.len(), 3);
            engine.discard();
            for letter in fields[0].chars() {
                let start = Instant::now();
                engine.handle(Key::Letter(letter));
                if round > 0 {
                    latencies.push(start.elapsed().as_nanos() as u64);
                }
            }
            if round == 0 {
                let frame = engine.frame();
                let words: Vec<_> = frame.rows.iter().map(|row| row.text.clone()).collect();
                let accepted: Vec<_> = fields[1].split('|').collect();
                let rank = words
                    .iter()
                    .position(|word| accepted.contains(&word.as_str()))
                    .map_or(10, |index| index + 1);
                cases.push(
                    json!({"code":fields[0],"accepted":accepted,"split":fields[2],
                                  "rank":rank,"candidates":words}),
                );
            }
        }
    }
    latencies.sort_unstable();
    let groups: Vec<_> = ["main", "holdout"]
        .iter()
        .map(|group| {
            let rows: Vec<_> = cases
                .iter()
                .filter(|case| case["split"] == *group)
                .collect();
            json!({"split":group,"cases":rows.len(),
            "top1":rows.iter().filter(|row| row["rank"] == 1).count(),
            "top3":rows.iter().filter(|row| row["rank"].as_u64().unwrap() <= 3).count(),
            "top9":rows.iter().filter(|row| row["rank"].as_u64().unwrap() <= 9).count()})
        })
        .collect();
    let mut regressions = Vec::new();
    let mut rank_changes = Vec::new();
    if let Some(baseline) = args.get(2) {
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(baseline).expect("baseline file"))
                .expect("baseline JSON");
        let old = baseline["cases"].as_array().expect("baseline cases");
        assert_eq!(old.len(), cases.len(), "fixture set changed");
        for (before, after) in old.iter().zip(&cases) {
            for field in ["code", "accepted", "split"] {
                assert_eq!(before[field], after[field], "fixture changed");
            }
            let old_rank = before["rank"].as_u64().unwrap();
            let new_rank = after["rank"].as_u64().unwrap();
            if new_rank > old_rank {
                rank_changes
                    .push(json!({"code":after["code"], "before":old_rank, "after":new_rank}));
            }
            // 词库迁移允许前三内的同音顺序变化；不能把可见词挤出原有可用范围。
            if (old_rank <= 3 && new_rank > 3) || (old_rank <= 9 && new_rank > 9) {
                regressions.push(after["code"].clone());
            }
        }
        let old_first = old.iter().filter(|row| row["rank"] == 1).count();
        let new_first = cases.iter().filter(|row| row["rank"] == 1).count();
        if new_first < old_first {
            regressions.push(json!("aggregate first-choice accuracy"));
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"load_us":load_us,
        "key_samples":latencies.len(),"warm_key_p50_ns":latencies[latencies.len()/2],
        "warm_key_p95_ns":latencies[latencies.len()*95/100],"groups":groups,
        "regressions":regressions,"rank_changes":rank_changes,"cases":cases}))
        .unwrap()
    );
    assert!(regressions.is_empty(), "candidate ranking regressed");
}
