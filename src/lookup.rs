// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

use std::collections::BTreeSet;
use std::fmt;
use std::fs::{read, read_dir};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::path::{Path, PathBuf};

use crate::{convert_png_to_rust_variables, PngToWasm4SrcError, RustVariables};

/// A module containing sprites
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Module {
    name: String,
    sprite_paths: BTreeSet<PathBuf>,
    submodules: BTreeSet<Module>,
}

/// A module containing Rust variables corresponding to sprites
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsedModule {
    name: String,
    variables: BTreeSet<RustVariables>,
    submodules: BTreeSet<ParsedModule>,
}

impl Module {
    /// Create a module from its name, its sprite paths, and its submodules
    pub fn new<R, S, T>(name: R, sprite_paths: S, submodules: T) -> Self
    where
        R: Into<String>,
        S: IntoIterator<Item = PathBuf>,
        T: IntoIterator<Item = Module>,
    {
        Self {
            name: name.into(),
            sprite_paths: sprite_paths.into_iter().collect(),
            submodules: submodules.into_iter().collect(),
        }
    }

    /// Parse the sprites in the module
    ///
    /// Parse all the sprites in the module and generate their Rust variables.
    pub fn parse(self) -> Result<ParsedModule, PngToWasm4SrcError> {
        let variables = self
            .sprite_paths
            .into_iter()
            .map(|path| {
                let name = path
                    .file_stem()
                    .ok_or(PngToWasm4SrcError::FileWithoutStem)?
                    .to_str()
                    .ok_or(PngToWasm4SrcError::NonUtf8Path)?;
                let bytes = read(&path)?;
                let rust_variables = convert_png_to_rust_variables(name, &bytes)?;
                Ok(rust_variables)
            })
            .collect::<Result<BTreeSet<RustVariables>, PngToWasm4SrcError>>()?;

        let submodules = self
            .submodules
            .into_iter()
            .map(|submodule| submodule.parse())
            .collect::<Result<BTreeSet<ParsedModule>, PngToWasm4SrcError>>()?;

        let parsed_module = ParsedModule::new(self.name, variables, submodules);

        Ok(parsed_module)
    }

    /// Flatten the module
    ///
    /// Flatten the module so that all sprites are defined in the top-level
    /// module.
    pub fn flatten(self) -> Self {
        let mut sprite_paths = self.sprite_paths;

        for submodule in self.submodules {
            let mut flattened_submodule = submodule.flatten();
            sprite_paths.append(&mut flattened_submodule.sprite_paths);
        }

        Self {
            name: self.name,
            sprite_paths,
            submodules: BTreeSet::default(),
        }
    }

    /// Generate instructions for cargo build
    ///
    /// Build scripts communicate with cargo by printing instructions starting
    /// with `cargo:` to standard output.
    /// This function generates a list of instructions to force a rebuild when
    /// source PNGs are modified.
    pub fn generate_cargo_build_instructions<W>(&self, output: &mut W) -> Result<(), fmt::Error>
    where
        W: std::fmt::Write,
    {
        for path in &self.sprite_paths {
            writeln!(output, "cargo:rerun-if-changed={}", path.display())?;
        }
        for submodule in &self.submodules {
            submodule.generate_cargo_build_instructions(output)?;
        }
        Ok(())
    }
}

impl ParsedModule {
    /// Create a module from its name, its variables, and its submodules
    pub fn new<R, S, T>(name: R, variables: S, submodules: T) -> Self
    where
        R: Into<String>,
        S: IntoIterator<Item = RustVariables>,
        T: IntoIterator<Item = ParsedModule>,
    {
        Self {
            name: name.into(),
            variables: variables.into_iter().collect(),
            submodules: submodules.into_iter().collect(),
        }
    }
}

impl fmt::Display for ParsedModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_parsed_module_with_indentation(self, 0, f).map_err(|_| fmt::Error)
    }
}

/// Build a sprite module tree from a directory containing sprites
pub fn build_sprite_modules_tree<P>(dir: P) -> Result<Module, PngToWasm4SrcError>
where
    P: AsRef<Path>,
{
    let dir: &Path = dir.as_ref();
    if dir.is_dir() {
        let module_name = dir
            .file_name()
            .ok_or_else(|| IoError::new(IoErrorKind::InvalidInput, "File without a name"))?
            .to_str()
            .ok_or(PngToWasm4SrcError::NonUtf8Path)?;

        // Process files first
        let files = read_dir(dir)?
            .map(|entry| {
                let entry = entry?;
                let path = entry.path();
                Ok(path)
            })
            .collect::<Result<Vec<PathBuf>, PngToWasm4SrcError>>()?
            .into_iter()
            .filter(|path| path.is_file())
            .filter(|path| path.extension().map(|s| s == "png").unwrap_or(false));

        // Then recurse into directories
        let submodules = read_dir(dir)?
            .map(|entry| {
                let entry = entry?;
                let path = entry.path();
                Ok(path)
            })
            .collect::<Result<Vec<PathBuf>, PngToWasm4SrcError>>()?
            .into_iter()
            .filter(|path| path.is_dir())
            .map(|path| build_sprite_modules_tree(&path))
            .collect::<Result<Vec<Module>, PngToWasm4SrcError>>()?
            .into_iter()
            .filter(|submodule| !submodule.sprite_paths.is_empty());

        let module = Module::new(module_name, files, submodules);
        return Ok(module);
    }

    Err(PngToWasm4SrcError::IoError(IoError::new(
        IoErrorKind::InvalidInput,
        "Not a directory",
    )))
}

fn write_parsed_module_with_indentation(
    module: &ParsedModule,
    level: usize,
    f: &mut fmt::Formatter,
) -> Result<(), PngToWasm4SrcError> {
    let mod_prefix = vec![32_u8; 4 * level];
    let mod_prefix = String::from_utf8(mod_prefix).expect("Cannot create string");
    let prefix = vec![32_u8; 4 * (level + 1)];
    let prefix = String::from_utf8(prefix).expect("Cannot create string");

    writeln!(f, "{}pub mod {} {{", mod_prefix, module.name)?;

    for rust_variables in &module.variables {
        let rust_code = rust_variables.to_string();
        for line in rust_code.split('\n') {
            if !line.is_empty() {
                writeln!(f, "{}pub {}", prefix, line)?;
            }
        }
        writeln!(f)?;
    }

    for submodule in &module.submodules {
        write_parsed_module_with_indentation(submodule, level + 1, f)?;
    }

    writeln!(f, "{}}}\n", mod_prefix)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn module_flatten() {
        let five = Module::new("five", vec![PathBuf::from("/five")], Vec::default());
        let four = Module::new("four", vec![PathBuf::from("/four")], Vec::default());
        let three = Module::new("three", vec![PathBuf::from("/three")], vec![four, five]);
        let two = Module::new("two", vec![PathBuf::from("/two")], Vec::default());
        let one = Module::new("one", vec![PathBuf::from("/one")], vec![two, three]);

        let flattened_module = one.flatten();

        let expected = Module::new(
            "one",
            vec![
                PathBuf::from("/one"),
                PathBuf::from("/two"),
                PathBuf::from("/three"),
                PathBuf::from("/four"),
                PathBuf::from("/five"),
            ],
            Vec::default(),
        );

        assert_eq!(flattened_module, expected);
    }

    #[test]
    fn module_generate_cargo_build_instructions() -> Result<()> {
        let five = Module::new("five", vec![PathBuf::from("/five")], Vec::default());
        let four = Module::new("four", vec![PathBuf::from("/four")], Vec::default());
        let three = Module::new("three", vec![PathBuf::from("/three")], vec![four, five]);
        let two = Module::new("two", vec![PathBuf::from("/two")], Vec::default());
        let one = Module::new("one", vec![PathBuf::from("/one")], vec![two, three]);

        let mut buffer = String::default();

        one.generate_cargo_build_instructions(&mut buffer)?;

        let expected = "cargo:rerun-if-changed=/one
cargo:rerun-if-changed=/three
cargo:rerun-if-changed=/five
cargo:rerun-if-changed=/four
cargo:rerun-if-changed=/two
";

        assert_eq!(buffer, expected);

        Ok(())
    }
}
