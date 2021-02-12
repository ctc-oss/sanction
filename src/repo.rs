use git2::Repository;
use std::path::{Path, PathBuf};

pub fn latest(url: &str, dir: &Option<String>) -> Result<(String, String), String> {
    let repodir: PathBuf = match dir {
        Some(d) => Path::new(d).to_path_buf(),
        None => tmp_repo_dir(url),
    };
    if !Path::new(repodir.as_path()).join(".git").exists() {
        clone(url, repodir.as_path())
    } else {
        pull(repodir.as_path())
    }
}

/// (path,sha) on ok, message on fail
fn clone(url: &str, path: &Path) -> Result<(String, String), String> {
    match Repository::clone(url, path) {
        Ok(r) => {
            let sha = r.revparse_single("HEAD").unwrap().id().to_string();
            Ok((path.display().to_string(), sha))
        }
        Err(e) => Err(format!("failed to clone: {}", e)),
    }
}

/// (path,sha) on ok, mesage on fail
fn pull(path: &Path) -> Result<(String, String), String> {
    match Repository::open(path) {
        Ok(r) => {
            let sha = r.revparse_single("HEAD").unwrap().id().to_string();
            Ok((path.display().to_string(), sha))
        }
        Err(e) => Err(format!("failed to pull: {}", e)),
    }
}

fn tmp_repo_dir(url: &str) -> PathBuf {
    Path::new("/tmp").join(
        url.trim_end_matches(".git")
            .split('/')
            .last()
            .unwrap()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmp_repo_dir_name() {
        let a = "https://github.com/foo/bar";
        let b = "https://github.com/foo/bar.git";

        assert_eq!(tmp_repo_dir(a), Path::new("/tmp/bar"));
        assert_eq!(tmp_repo_dir(b), Path::new("/tmp/bar"));
    }
}
