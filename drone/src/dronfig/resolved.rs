use std::{collections::BTreeMap, fs, path::Path, sync::Mutex};

use ignore::{WalkBuilder, WalkState};

use super::file::{DronfigFile, FoundryResource};
use crate::error::Error;

#[derive(Debug)]
pub struct Dronfig {
    pub resources: Vec<FoundryResource>,
}

impl Dronfig {
    /// Resolves home declarations first, then overlays declarations from cwd.
    pub fn resolve(
        home: &Path,
        cwd: &Path,
        recursive: bool,
        ignore: bool,
    ) -> Result<(Self, Vec<Error>), Error> {
        let mut resources = BTreeMap::new();
        let mut errors = Vec::new();

        for directory in [home, cwd]
            .into_iter()
            .take(if cwd == home { 1 } else { 2 })
        {
            let declarations = Mutex::new(Vec::new());
            let mut walker = WalkBuilder::new(directory);
            walker
                .max_depth((!recursive).then_some(1))
                .standard_filters(ignore);

            // Discovery
            walker.build_parallel().run(|| {
                Box::new(|entry| {
                    let path = match entry {
                        Ok(entry) => entry.into_path(),
                        Err(error) => {
                            declarations.lock().unwrap().push(Err(Error::Config {
                                message: format!(
                                    "could not search for resource declarations: {error}"
                                ),
                                help: format!(
                                    "check that {} exists and is readable",
                                    directory.display()
                                ),
                            }));
                            return WalkState::Continue;
                        }
                    };
                    if !path.is_file()
                        || !path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .is_some_and(|name| name.ends_with(".drone.toml"))
                    {
                        return WalkState::Continue;
                    }

                    let content = fs::read_to_string(&path).map_err(|error| Error::Config {
                        message: format!("could not read resource declarations: {error}"),
                        help: format!("check that {} is readable", path.display()),
                    });
                    let result = content.and_then(|content| {
                        toml::from_str::<DronfigFile>(&content).map_err(|error| Error::Config {
                            message: format!("invalid resource declarations: {error}"),
                            help: format!("fix the TOML in {}", path.display()),
                        })
                    });
                    declarations
                        .lock()
                        .unwrap()
                        .push(result.map(|file| (path, file.resources)));
                    WalkState::Continue
                })
            });

            // Validation
        }
        Ok((
            Self {
                resources: resources.into_values().collect(),
            },
            errors,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    #[test]
    fn cwd_overrides_global_resource() {
        let root = std::env::temp_dir().join(format!(
            "drone-dronfig-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let global = root.join("global");
        let cwd = root.join("cwd");
        fs::create_dir_all(&global).unwrap();
        fs::create_dir_all(&cwd).unwrap();
        fs::write(
            global.join("global.drone.toml"),
            r#"
            [[resources]]
            kind = "ontology"
            name = "cbre"
            rid = "ri.ontology.global"
            "#,
        )
        .unwrap();
        fs::write(
            cwd.join("local.drone.toml"),
            r#"
            [[resources]]
            kind = "ontology"
            name = "cbre"
            rid = "ri.ontology.local"
            "#,
        )
        .unwrap();
        let (dronfig, errors) = Dronfig::resolve(&global, &cwd, false, false).unwrap();

        assert!(errors.is_empty());
        assert_eq!(dronfig.resources.len(), 1);
        assert_eq!(dronfig.resources[0].rid.0, "ri.ontology.local");
        assert_eq!(dronfig.resources[0].deployment, None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recursive_controls_nested_discovery() {
        let root = std::env::temp_dir().join(format!(
            "drone-dronfig-recursive-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let home = root.join("home");
        let cwd = root.join("cwd");
        let nested = cwd.join("nested");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&nested).unwrap();
        fs::write(
            nested.join("nested.drone.toml"),
            r#"
            [[resources]]
            kind = "space"
            name = "nested"
            rid = "ri.compass.nested"
            "#,
        )
        .unwrap();

        assert!(
            Dronfig::resolve(&home, &cwd, false, false)
                .unwrap()
                .0
                .resources
                .is_empty()
        );
        assert_eq!(
            Dronfig::resolve(&home, &cwd, true, false)
                .unwrap()
                .0
                .resources
                .len(),
            1
        );
        fs::remove_dir_all(root).unwrap();
    }
}
