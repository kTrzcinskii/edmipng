use std::{path::PathBuf, str::FromStr};

use anyhow::Error;

pub enum Source {
    Path(PathBuf),
    Url(String),
}

impl Source {
    /// If source is path just returns it, otherwise return path to non-existing file with name based on url and date
    pub fn get_output_file_path(&self) -> &PathBuf {
        match self {
            Source::Path(path) => path,
            // TODO: we wanna do something like:
            // if url is www.example.com/files/<name>.png
            // then we create "<date>_<name>.png"
            // we probably can have some predefined directory to put it in, which we can load from env
            Source::Url(_) => todo!(),
        }
    }
}

impl FromStr for Source {
    type Err = Error;

    // We do it very simple - instead of using some crazy regex like this one:
    // https://stackoverflow.com/questions/161738/what-is-the-best-regular-expression-to-check-if-a-string-is-a-valid-url
    // we just check if given string points to exsiting file
    // - if it does -> it's a path
    // - if it doesn't -> we assume it's an url
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = PathBuf::from(s);
        match p.try_exists() {
            Ok(true) => Ok(Source::Path(p)),
            Ok(false) => Ok(Source::Url(s.into())),
            Err(_) => Ok(Source::Url(s.into())),
        }
    }
}
