use anyhow::{format_err, Context, Error};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct Build {
    out_dir: PathBuf,
}

impl Build {
    pub fn new() -> Result<Self> {
        let out_dir = env::var_os("OUT_DIR")
            .context("could not find the output directory, are we running in a build.rs script?")?;
        // Assume we are in a build script. For other uses the caller is responsible for handling
        // stdout.
        rerun_if_changed("build.rs");
        Ok(Self {
            out_dir: out_dir.into(),
        })
    }

    /// Create a file with the given contents that will be available to the source of this crate at
    /// compiletime.
    ///
    /// You can then include it with `include_str` et al.
    pub fn write_file(&self, path: &Path, contents: impl AsRef<[u8]>) -> Result {
        let path = self.out_dir.join(path);
        fs::write(&path, contents.as_ref()).context("could not write file to `out` directory")?;
        Ok(())
    }

    /// Tell cargo to re-run the build process if the file at `path` has changed.
    pub fn rerun_if_changed(&self, path: impl AsRef<Path>) -> Result {
        rerun_if_changed(utf8_cargo_path(path.as_ref())?);
        Ok(())
    }
}

fn utf8_cargo_path(path: &Path) -> Result<&str> {
    path.to_str().ok_or(format_err!(
        "cargo requires that paths are utf8, but \"{}\" is not",
        path.display()
    ))
}

fn rerun_if_changed(s: &str) {
    println!("cargo:rerun-if-changed={}", s);
}
