extern crate unikko;

mod canon_fixtures;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use canon_fixtures::*;

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
            let test_case = test_cases.get(case_name.as_str()).unwrap();
            count_all += 1;
            let input_clone = test_case.input.clone();
            let actual = unikko::textile_to_html(input_clone.clone())?;
            if actual == test_case.expect {
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
        }
        assert_eq!(count_all_pass, count_all, "{} failed. See set fixture_set", count_all_fail);
    }

    Ok(())
}
