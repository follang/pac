use std::env;
use std::io;
use std::path::{Path, PathBuf};

use crate::driver::{self, Config, Flavor};
use crate::env::Env;
use crate::parser;

use super::support::{collect_fixture_dirs, manifest_list_values, manifest_value, read_file};

struct FullAppCase {
    path: PathBuf,
    flavor: AppFlavor,
    mode: AppMode,
    entry: PathBuf,
    include_dirs: Vec<PathBuf>,
    tags: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
enum AppFlavor {
    Core,
    Gnu,
    Clang,
}

#[derive(Copy, Clone, Debug)]
enum AppMode {
    TranslationUnit,
    Driver,
}

impl FullAppCase {
    fn from_dir(path: PathBuf) -> io::Result<FullAppCase> {
        let manifest_path = path.join("fixture.toml");
        let manifest = read_file(&manifest_path)?;

        let mut flavor = AppFlavor::Core;
        let mut mode = AppMode::TranslationUnit;
        let mut entry = None;
        let mut include_dirs = Vec::new();
        let mut tags = Vec::new();

        for line in manifest.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(value) = manifest_value(line, "mode") {
                mode = match value {
                    "translation_unit" => AppMode::TranslationUnit,
                    "driver" => AppMode::Driver,
                    _ => panic!("{}: unsupported mode `{}`", manifest_path.display(), value),
                };
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

            if let Some(values) = manifest_list_values(line, "include_dirs") {
                include_dirs = values.into_iter().map(PathBuf::from).collect();
            }

            if let Some(values) = manifest_list_values(line, "tags") {
                tags = values;
            }
        }

        Ok(FullAppCase {
            path: path,
            flavor: flavor,
            mode: mode,
            entry: entry.unwrap_or_else(|| PathBuf::from("main.c")),
            include_dirs: include_dirs,
            tags: tags,
        })
    }

    fn run(&self) -> Result<(), parser::ParseError> {
        let source_path = self.path.join(&self.entry);
        match self.mode {
            AppMode::TranslationUnit => {
                let source = read_file(&source_path).expect("reading full app source");
                let mut env = match self.flavor {
                    AppFlavor::Core => Env::with_core(),
                    AppFlavor::Gnu => Env::with_gnu(),
                    AppFlavor::Clang => Env::with_clang(),
                };
                parser::translation_unit(source.trim_end(), &mut env).map(|_| ())
            }
            AppMode::Driver => {
                let mut config = config_for(self.flavor);
                config.flavor = flavor_for(self.flavor);
                for include_dir in &self.include_dirs {
                    config
                        .cpp_options
                        .push(format!("-I{}", self.path.join(include_dir).display()));
                }
                driver::parse(&config, &source_path)
                    .map(|_| ())
                    .map_err(driver_error_to_parse_error)
            }
        }
    }

    fn matches_filters(&self, path_filter: Option<&str>, tag_filter: Option<&str>) -> bool {
        if let Some(path_filter) = path_filter {
            let path = self.path.display().to_string();
            if !path.contains(path_filter) {
                return false;
            }
        }

        if let Some(tag_filter) = tag_filter {
            if !self.tags.iter().any(|tag| tag == tag_filter) {
                return false;
            }
        }

        true
    }

    fn describe(&self) -> String {
        format!(
            "{} [mode={:?}, flavor={:?}]",
            self.path.display(),
            self.mode,
            self.flavor
        )
    }
}

fn config_for(flavor: AppFlavor) -> Config {
    match flavor {
        AppFlavor::Clang => Config::with_clang(),
        AppFlavor::Core | AppFlavor::Gnu => Config::with_gcc(),
    }
}

fn flavor_for(flavor: AppFlavor) -> Flavor {
    match flavor {
        AppFlavor::Core => Flavor::StdC11,
        AppFlavor::Gnu => Flavor::GnuC11,
        AppFlavor::Clang => Flavor::ClangC11,
    }
}

fn driver_error_to_parse_error(error: driver::Error) -> parser::ParseError {
    match error {
        driver::Error::SyntaxError(err) => parser::ParseError {
            line: err.line,
            column: err.column,
            offset: err.offset,
            expected: err.expected,
        },
        driver::Error::PreprocessorError(err) => {
            panic!("preprocessor error: {}", err);
        }
    }
}

#[test]
fn full_app_main() {
    let mut case_paths = Vec::new();
    collect_fixture_dirs(Path::new("test/full_apps"), &mut case_paths);
    assert!(!case_paths.is_empty(), "expected at least one full app fixture");
    let path_filter = env::var("FULL_APP_FILTER").ok();
    let tag_filter = env::var("FULL_APP_TAG").ok();

    let failed = case_paths
        .iter()
        .map(|path| FullAppCase::from_dir(path.to_path_buf()).expect("loading full app fixture"))
        .filter(|case| case.matches_filters(path_filter.as_deref(), tag_filter.as_deref()))
        .filter_map(|case| match case.run() {
            Ok(()) => None,
            Err(err) => Some(format!("{}: {}", case.describe(), err)),
        })
        .collect::<Vec<_>>();

    if !failed.is_empty() {
        panic!("{} full app cases failed:\n{}", failed.len(), failed.join("\n"));
    }
}

#[test]
fn full_app_filter_matches_path_and_tag() {
    let case = FullAppCase {
        path: PathBuf::from("test/full_apps/synthetic/single_file/state_machine"),
        flavor: AppFlavor::Gnu,
        mode: AppMode::TranslationUnit,
        entry: PathBuf::from("main.c"),
        include_dirs: Vec::new(),
        tags: vec!["synthetic".to_owned(), "single_file".to_owned()],
    };

    assert!(case.matches_filters(None, None));
    assert!(case.matches_filters(Some("state_machine"), None));
    assert!(case.matches_filters(None, Some("synthetic")));
    assert!(case.matches_filters(Some("single_file"), Some("synthetic")));
    assert!(!case.matches_filters(Some("mini_http"), None));
    assert!(!case.matches_filters(None, Some("external")));
}
