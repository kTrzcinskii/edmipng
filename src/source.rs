use std::{
    path::PathBuf,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Error, Result};
use reqwest::Url;

pub enum Source {
    Path(PathBuf),
    Url(Url),
}

impl Source {
    /// If source is path just returns it, otherwise return path to non-existing file with name based on url and date
    pub fn get_output_file_path(&self) -> Result<PathBuf> {
        match self {
            Source::Path(path) => Ok(path.clone()),
            Source::Url(url) => {
                let url_name = match url.path_segments() {
                    Some(segments) => segments.last().unwrap_or("png_file"),
                    None => "png_file",
                };
                let without_png_suffix = url_name.strip_suffix(".png").unwrap_or(url_name);
                let current_time = SystemTime::now();
                let epoch_time = current_time
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
                    .to_string();
                // TODO: For now just filename, later add directory which can be set in env
                let full_name = format!("{}_{}.png", without_png_suffix, epoch_time);
                let path = PathBuf::from_str(&full_name)?;
                Ok(path)
            }
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
            Ok(false) => Ok(Source::Url(Url::parse(s)?)),
            Err(_) => Ok(Source::Url(Url::parse(s)?)),
        }
    }
}
