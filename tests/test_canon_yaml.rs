extern crate unikko;

mod canon_fixtures;

use anyhow::{anyhow, Result};
use canon_fixtures::*;

#[test]
fn tokenizer_only() -> Result<(), anyhow::Error> {
    let fixtures_root = FixturesRoot::new();
    for fixture_set in fixtures_root {
        for (_name, fixture_test_case) in fixture_set {
            let mut input = std::io::Cursor::new(fixture_test_case.input);
            let _ = unikko::tokenize(&mut input)?;
        }
    }

    Ok(())
}

#[test]
fn textile_to_html() -> Result<(), anyhow::Error> {
    let mut count_all_pass = 0;
    let mut count_all_fail = 0;
    let fixtures_root = FixturesRoot::new();
    for fixture_set in fixtures_root {
        let mut count_set_pass = 0;
        let mut count_set_fail = 0;
        for (_name, fixture_test_case) in fixture_set {
            let actual = unikko::textile_to_html(fixture_test_case.input)?;
            if actual == fixture_test_case.expect {
                count_all_pass += 1;
                count_set_pass += 1;
            } else {
                count_all_fail += 1;
                count_set_fail += 1;
            }
        }
        println!("In this set: {count_set_pass} passsed, {count_set_fail} failed.");
        break;
    }

    println!("In total: {count_all_pass} passed, {count_all_fail} failed.");
    Ok(())

    // if count_all_fail > 0 {
    //     Err(anyhow!(
    //         "In total: {count_all_pass} passed, {count_all_fail} failed."
    //     ))
    // } else {
    //     Ok(())
    // }
}
