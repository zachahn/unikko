extern crate unikko;

mod canon_fixtures;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use canon_fixtures::*;
use lol_html::{doc_text, rewrite_str, RewriteStrSettings};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

fn normalized(fragment: &str) -> String {
    static REMOVE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());
    let html = rewrite_str(
        fragment,
        RewriteStrSettings {
            document_content_handlers: vec![doc_text!(|t| {
                // println!("normalize doc content: {:?}", t.as_str());
                if REMOVE_WHITESPACE.is_match(t.as_str()) {
                    t.remove();
                }
                Ok(())
            })],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    return html;
}

#[test]
fn textile_to_html() {
    let mut count_total_cases = 0;
    let mut count_total_passed = 0;
    let mut count_total_crashes = 0;
    let mut count_total_skipped = 0;
    let mut printed_first_failure = false;
    let mut passed_sets = Vec::<String>::new();
    let mut skip_cases = HashSet::new();
    skip_cases.insert("Basic Ordered List");
    skip_cases.insert("Basic Unordered lists");
    for fixture_set in FixturesRoot::new() {
        let mut count_set_cases = 0;
        let mut count_set_passed = 0;
        let set_name = fixture_set.name;
        let test_cases = fixture_set.cases;
        let test_case_names: Vec<String> = {
            let mut unsorted: Vec<_> = test_cases.keys().cloned().collect();
            unsorted.sort();
            unsorted
        };
        for case_name in test_case_names {
            count_total_cases += 1;
            count_set_cases += 1;
            let test_case = test_cases.get(case_name.as_str()).unwrap();
            if skip_cases.contains(case_name.as_str()) {
                count_total_skipped += 1;
                continue;
            }
            if matches!(test_case.class, Some(_)) || matches!(test_case.setup, Some(_)) {
                count_total_skipped += 1;
                continue;
            }
            let actual = unikko::textile_to_html(test_case.input.clone());
            if matches!(actual, Err(_)) {
                count_total_crashes += 1;
                continue;
            }
            let actual = actual.unwrap();
            if normalized(actual.as_str()) == normalized(test_case.expect.as_str()) {
                count_total_passed += 1;
                count_set_passed += 1;
                continue;
            }
            if !printed_first_failure {
                println!("➡️   FAILURE:\n- set: {}\n- case: {}\n", set_name, case_name);
                if let Some(ref note) = test_case.note {
                    println!("➡️   NOTE:\n{}\n", note);
                }
                if let Some(ref setup) = test_case.setup {
                    println!("➡️   SETUP:\n{:?}\n", setup);
                }
                println!("➡️   INPUT:\n{}\n\n", test_case.input);
                println!("➡️   EXPECTED:\n{}\n\n", test_case.expect);
                println!("➡️   ACTUAL:\n{}\n", actual);
                println!(
                    "➡️   NODES:\n{:?}",
                    unikko::textile_to_tree(test_case.input.clone())
                );
                printed_first_failure = true;
            }
        }
        if count_set_cases == count_set_passed {
            passed_sets.push(set_name.clone());
        }
    }
    assert_eq!(count_total_crashes, 0);
    let count_total_failed = count_total_cases - count_total_passed;
    assert_eq!(
        count_total_passed, count_total_cases,
        "{count_total_failed}. passed sets: {passed_sets:?}. skipped: {count_total_skipped}, passed: {count_total_passed}. total: {count_total_cases}"
    );
}
