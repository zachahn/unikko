use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;

pub struct Fixture {
    pub path: PathBuf,
    pub name: String,
    pub test_case: FixtureTestCase,
}

impl Fixture {
    pub fn filename(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }
}

pub struct Fixtures {
    fixtures_root: FixturesRoot,
    current_cases: Option<std::collections::hash_map::IntoIter<String, FixtureTestCase>>,
    current_path: Option<PathBuf>,
}

impl Fixtures {
    pub fn new() -> Self {
        Self {
            fixtures_root: FixturesRoot::new(),
            current_cases: None,
            current_path: None,
        }
    }
}

impl Iterator for Fixtures {
    type Item = Fixture;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Initialize current_cases if needed
            if self.current_cases.is_none() {
                match self.fixtures_root.next() {
                    None => return None,
                    Some(FixtureSet { cases, path }) => {
                        self.current_cases = Some(cases.into_iter());
                        self.current_path = Some(path);
                    }
                }
            }

            // Try to get next test case from current fixture set
            match self.current_cases.as_mut().unwrap().next() {
                Some((name, test_case)) => {
                    return Some(Fixture {
                        path: self.current_path.clone().unwrap(),
                        name: name,
                        test_case,
                    })
                }
                None => {
                    // Current fixture set exhausted, move to next one
                    self.current_cases = None;
                    // Loop continues to load next fixture set
                }
            }
        }
    }
}

pub struct FixturesRoot {
    dir_iter: std::vec::IntoIter<DirEntry>,
}

pub struct FixtureSet {
    pub cases: HashMap<String, FixtureTestCase>,
    pub path: PathBuf,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum FixtureSetup {
    setRestricted,
    setLite,
    setImages,
    setLinkRelationShip,
    setDimensionlessImages,
    setBlockTags,
    setImagePrefix,
    setLinkPrefix,
    setRawBlocks,
    setLineWrap,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureTestCase {
    pub note: Option<String>,
    pub class: Option<String>,
    pub input: String,
    pub expect: String,
    pub setup: Option<Vec<HashMap<FixtureSetup, String>>>,
}

impl FixturesRoot {
    pub fn new() -> Self {
        let mut fixtures_root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fixtures_root_path.push("canon/test/fixtures");

        let mut fixture_paths: Vec<_> = fixtures_root_path
            .read_dir()
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        fixture_paths.sort_by_key(|path| path.path());

        Self {
            dir_iter: fixture_paths.into_iter(),
        }
    }
}

impl Iterator for FixturesRoot {
    type Item = FixtureSet;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir_iter.next() {
            None => None,
            Some(current) => {
                let path = current.path();
                // let name = path.file_name().unwrap().to_str().unwrap().to_string();
                // println!("📁 📁 📁  {}", path.display());

                let contents = std::fs::read_to_string(&path).unwrap();
                let fixture_test_cases: HashMap<String, FixtureTestCase> =
                    serde_yaml::from_str(&contents).unwrap();
                Some(Self::Item {
                    cases: fixture_test_cases,
                    path: path,
                })
            }
        }
    }
}
