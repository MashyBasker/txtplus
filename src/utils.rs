use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use shellexpand;

pub fn abspath(path: &str) -> Option<String> {
    shellexpand::full(path)
        .ok()
        .and_then(|f| Path::new(OsStr::new(f.as_ref())).canonicalize().ok())
        .and_then(|p| p.into_os_string().into_string().ok())
}

pub fn write_filepath(path: &str) -> Option<String> {
    let abs_path: String = abspath(path).unwrap();
    let mut path_components: Vec<&str> = abs_path.split('/').collect::<Vec<&str>>();
    let file: Option<&str> = path_components.pop();
    let dirpath = path_components.join("/");

    if let Some(f) = file {
        f.split('.')
            .collect::<Vec<&str>>()
            .first()
            .map(|s| format!("{}/{}.plus.txt", dirpath, s.to_string()))
    } else {
        None
    }
}

pub fn append_to_file(f_path: &String, line: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new().append(true).create(true).open(f_path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_filename() {
        assert_eq!(
            write_filepath("/home/okabe/projects/txtplus/testdata/mock/test.txt"),
            Some("/home/okabe/projects/txtplus/testdata/mock/test.plus.txt".to_string())
        );
        assert_eq!(
            write_filepath("~/projects/txtplus/testdata/mock/test.txt"),
            Some("/home/okabe/projects/txtplus/testdata/mock/test.plus.txt".to_string())
        );
        assert_eq!(
            write_filepath("./testdata/mock/test.txt"),
            Some("/home/okabe/projects/txtplus/testdata/mock/test.plus.txt".to_string())
        );
    }

    #[test]
    #[ignore]
    fn test_abspath() {
        assert_eq!(
            abspath("~/projects"),
            Some("/home/okabe/projects".to_string())
        );
        assert_eq!(
            abspath("../coccinelle"),
            Some("/home/okabe/projects/coccinelle".to_string())
        );
        assert_eq!(abspath("~"), Some("/home/okabe".to_string()));
        assert_eq!(
            abspath("./target"),
            Some("/home/okabe/projects/txtplus/target".to_string())
        );
        assert_eq!(
            abspath("/home/okabe/projects"),
            Some("/home/okabe/projects".to_string())
        );
    }
}
