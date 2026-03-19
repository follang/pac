use std::io;
use std::path::{Path, PathBuf};

use crate::env::Env;
use crate::parser;

use super::support::{collect_fixture_dirs, manifest_value, read_file};

struct FullAppCase {
    path: PathBuf,
    flavor: AppFlavor,
    entry: PathBuf,
}

#[derive(Copy, Clone)]
enum AppFlavor {
    Core,
    Gnu,
    Clang,
}

impl FullAppCase {
    fn from_dir(path: PathBuf) -> io::Result<FullAppCase> {
        let manifest_path = path.join("fixture.toml");
        let manifest = read_file(&manifest_path)?;

        let mut flavor = AppFlavor::Core;
        let mut entry = None;

        for line in manifest.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(value) = manifest_value(line, "flavor") {
                flavor = match value {
                    "core" | "std" => AppFlavor::Core,
                    "gnu" => AppFlavor::Gnu,
                    "clang" => AppFlavor::Clang,
                    _ => panic!("{}: unsupported flavor `{}`", manifest_path.display(), value),
                };
            }

            if let Some(value) = manifest_value(line, "entry") {
                entry = Some(PathBuf::from(value));
            }
        }

        Ok(FullAppCase {
            path: path,
            flavor: flavor,
            entry: entry.unwrap_or_else(|| PathBuf::from("main.c")),
        })
    }

    fn run(&self) -> Result<(), parser::ParseError> {
        let source_path = self.path.join(&self.entry);
        let source = read_file(&source_path).expect("reading full app source");
        let mut env = match self.flavor {
            AppFlavor::Core => Env::with_core(),
            AppFlavor::Gnu => Env::with_gnu(),
            AppFlavor::Clang => Env::with_clang(),
        };

        parser::translation_unit(source.trim_end(), &mut env).map(|_| ())
    }
}

#[test]
fn full_app_main() {
    let mut case_paths = Vec::new();
    collect_fixture_dirs(Path::new("test/full_apps"), &mut case_paths);
    assert!(!case_paths.is_empty(), "expected at least one full app fixture");

    let failed = case_paths
        .iter()
        .map(|path| FullAppCase::from_dir(path.to_path_buf()).expect("loading full app fixture"))
        .filter(|case| case.run().is_err())
        .map(|case| case.path.display().to_string())
        .collect::<Vec<_>>();

    if !failed.is_empty() {
        panic!("{} full app cases failed: {:?}", failed.len(), failed);
    }
}
