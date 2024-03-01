extern crate unikko;

mod canon_fixtures;

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
