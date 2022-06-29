use std::fs;
use std::path::Path;

pub fn has_git(path: String) -> bool {
    Path::new(&path).exists()
}

pub fn is_local_git(path: String) -> bool {
    if !has_git(path.clone()) {
        return false;
    }

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(..) => {
            return false;
        }
    };

    if !contents.contains("url") {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_git_test() {
        assert_eq!(has_git("./".to_string()), true);
    }

    #[test]
    fn is_local_git_test() {
        assert_eq!(is_local_git("./".to_string()), false);
    }
}
