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

            // Discover and parse declarations in parallel.
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

            // Merge lower-priority declarations first so later declarations win.
            let mut declarations = declarations.into_inner().unwrap();
            declarations.sort_by(|left, right| match (left, right) {
                (Ok((left, _)), Ok((right, _))) => right
                    .components()
                    .count()
                    .cmp(&left.components().count())
                    .then_with(|| left.cmp(right)),
                (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                (Err(_), Err(_)) => std::cmp::Ordering::Equal,
            });

            for declaration in declarations {
                match declaration {
                    Ok((path, file)) => {
                        for resource in file {
                            let key = (resource.deployment.clone(), resource.name.clone());
                            let overriding = format!(
                                "{} takes precedence with kind `{}`, RID `{}`, alias {:?}, default {:?}",
                                path.display(),
                                resource.kind.as_ref(),
                                resource.rid.0,
                                resource.alias,
                                resource.default,
                            );
                            if let Some((shadowed_path, shadowed_resource)) =
                                resources.insert(key, (path.clone(), resource))
                            {
                                let deployment =
                                    shadowed_resource.deployment.as_deref().unwrap_or("default");
                                errors.push(Error::Config {
                                    message: format!(
                                        "resource `{}` in deployment `{deployment}` is overridden",
                                        shadowed_resource.name
                                    ),
                                    help: format!(
                                        "{} declares kind `{}`, RID `{}`, alias {:?}, default {:?}; {overriding}. Rename or remove one declaration",
                                        shadowed_path.display(),
                                        shadowed_resource.kind.as_ref(),
                                        shadowed_resource.rid.0,
                                        shadowed_resource.alias,
                                        shadowed_resource.default,
                                    ),
                                });
                            }
                        }
                    }
                    Err(error) => errors.push(error),
                }
            }
        }
        Ok((
            Self {
                resources: resources
                    .into_values()
                    .map(|(_, resource)| resource)
                    .collect(),
            },
            errors,
        ))
    }
}
