use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;

pub struct FixturesRoot {
    iter: std::vec::IntoIter<DirEntry>,
}

pub struct FixtureSet {
    pub cases: HashMap<String, FixtureTestCase>,
    pub name: String,
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
            iter: fixture_paths.into_iter(),
        }
    }
}

impl Iterator for FixturesRoot {
    type Item = FixtureSet;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(current) => {
                let path = current.path();
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                // println!("📁 📁 📁  {}", path.display());

                let contents = std::fs::read_to_string(path).unwrap();
                let fixture_test_cases: HashMap<String, FixtureTestCase> =
                    serde_yaml::from_str(&contents).unwrap();
                Some(Self::Item {
                    cases: fixture_test_cases,
                    name: name,
                })
            }
        }
    }
}
