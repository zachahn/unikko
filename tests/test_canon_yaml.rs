extern crate unikko;

mod canon_fixtures;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use canon_fixtures::*;
use lol_html::{doc_text, rewrite_str, RewriteStrSettings};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

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
    let fixtures = Fixtures::new();
    let mut passed: Vec<Fixture> = vec![];
    let mut skipped: Vec<Fixture> = vec![];
    let mut errored: Vec<Fixture> = vec![];
    let mut mismatched: Vec<Fixture> = vec![];

    for fixture in fixtures {
        if fixture.test_case.setup.is_some() {
            skipped.push(fixture);
            continue;
        }
        let actual = unikko::textile_to_html(fixture.test_case.input.as_str());
        if matches!(actual, Err(_)) {
            errored.push(fixture);
            continue;
        }
        let actual = actual.unwrap();
        if normalized(actual.as_str()) != normalized(fixture.test_case.expect.as_str()) {
            mismatched.push(fixture);
            continue;
        }
        passed.push(fixture);
    }

    if errored.len() > 0 || mismatched.len() > 0 {
        let mut error_examples: HashMap<PathBuf, (&Fixture, String)> = HashMap::new();
        for fixture in &errored {
            // Re-run to capture the error message
            if let Err(e) = unikko::textile_to_html(fixture.test_case.input.as_str()) {
                error_examples
                    .entry(fixture.path.clone())
                    .or_insert((fixture, e.to_string()));
            }
        }

        if !error_examples.is_empty() {
            println!("=== ERROR EXAMPLES (one per file) ===");
            for (_path, (fixture, error_msg)) in error_examples.iter() {
                println!("Test: {} -- {}", fixture.filename(), fixture.name);
                println!("Input:\n{}", fixture.test_case.input);
                println!("Expected:\n{}", fixture.test_case.expect);
                println!("Error:\n{}", error_msg);
                println!("{}", "=".repeat(80));
            }
        }

        let mut mismatch_examples: HashMap<PathBuf, &Fixture> = HashMap::new();
        for fixture in &mismatched {
            mismatch_examples
                .entry(fixture.path.clone())
                .or_insert(fixture);
        }

        if !mismatch_examples.is_empty() {
            println!("=== MISMATCH EXAMPLES (one per file) ===");
            for (_path, fixture) in mismatch_examples.iter() {
                println!("Test: {} -- {}", fixture.filename(), fixture.name);
                println!("Input:\n{}", fixture.test_case.input);
                println!("Expected:\n{}", fixture.test_case.expect);

                // Re-compute actual for this example (raw HTML, not normalized)
                let actual = unikko::textile_to_html(fixture.test_case.input.as_str()).unwrap();
                println!("Actual:\n{}", actual);
                println!("{}", "=".repeat(80));
            }
        }

        println!("Summary");
        println!("Passed: {}", passed.len());
        println!("Skipped: {}", skipped.len());
        println!("Errored: {}", errored.len());
        println!("Mismatched: {}", mismatched.len());
        panic!()
    }
}
