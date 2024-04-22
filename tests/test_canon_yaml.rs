extern crate unikko;
#[macro_use]
extern crate lazy_static;

mod canon_fixtures;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use canon_fixtures::*;
use lol_html::{doc_text, rewrite_str, RewriteStrSettings};
use regex::Regex;

lazy_static! {
    static ref NORMALIZATION_WHITESPACE: Regex = Regex::new(r"^\s+$").unwrap();
}

fn normalized(fragment: &str) -> String {
    let html = rewrite_str(
        fragment,
        RewriteStrSettings {
            document_content_handlers: vec![doc_text!(|t| {
                // println!("normalize doc content: {:?}", t.as_str());
                if NORMALIZATION_WHITESPACE.is_match(t.as_str()) {
                    t.remove();
                }
                Ok(())
            })],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    // println!("normalized: {}", html);
    return html;
}

#[test]
fn textile_to_html() -> Result<()> {
    let mut count_all_pass = 0;
    let mut count_all_fail = 0;
    let mut count_all = 0;
    let mut printed = false;
    for fixture_set in FixturesRoot::new() {
        let set_name = fixture_set.name.clone();
        let test_cases = fixture_set.cases;
        let mut test_case_names: Vec<String> = test_cases.keys().cloned().collect();
        test_case_names.sort();
        for case_name in test_case_names {
            count_all += 1;
            let test_case = test_cases.get(case_name.as_str()).unwrap();
            let input_clone = test_case.input.clone();
            let actual = unikko::textile_to_html(test_case.input.clone())?;
            if normalized(actual.as_str()) == normalized(test_case.expect.as_str()) {
                count_all_pass += 1;
            } else {
                if !printed {
                    println!("➡️   FAILURE:\n- set: {}\n- case: {}\n", set_name, case_name);
                    println!("➡️   INPUT:\n{}\n\n", input_clone);
                    println!("➡️   EXPECTED:\n{}\n\n", test_case.expect);
                    println!("➡️   ACTUAL:\n{}\n", actual);
                    printed = true;
                }
                count_all_fail += 1;
            }
            break;
        }
        assert_eq!(
            count_all_pass, count_all,
            "{}. passed: {}. total: {}. set: {}",
            count_all_fail, count_all_pass, count_all, set_name
        );
    }

    Ok(())
}
