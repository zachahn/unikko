extern crate unikko;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FixtureTestCase {
    note: Option<String>,
    input: String,
    expect: String,
}

#[test]
fn test_canon_yaml() -> Result<(), anyhow::Error> {
    let mut fixtures_dirpath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixtures_dirpath.push("canon/test/fixtures");

    let mut fixture_paths: Vec<_> = fixtures_dirpath.read_dir()?.map(|r| r.unwrap()).collect();
    fixture_paths.sort_by_key(|path| path.path());

    for fixture_path in fixture_paths {
        let contents = fs::read_to_string(fixture_path.path())?;
        let fixture_test_cases: HashMap<String, FixtureTestCase> = serde_yaml::from_str(&contents)?;
        for (name, fixture_test_case) in fixture_test_cases {
            println!("🔎 {:?}", name);
            println!("{:?}", fixture_test_case);
        }
    }

    Ok(())
}
