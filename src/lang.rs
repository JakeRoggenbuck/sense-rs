use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Lang {
    Python,
    Rust,
    Java,
    JS,
    Go,
    None,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn has_sub_file(path: &String, sub: &str) -> bool {
    println!("{}", &(path.to_owned() + &sub));
    Path::new(&(path.to_owned() + &sub)).exists()
}

pub fn get_lang(path: String) -> Lang {
    let mut path = path;
    if path.chars().nth(path.len() - 1).unwrap() != '/' {
        path.push('/');
    }

    if has_sub_file(&path, "setup.py") {
        return Lang::Python;
    } else if has_sub_file(&path, "package.json") {
        return Lang::JS;
    } else if has_sub_file(&path, "Cargo.toml") {
        return Lang::Rust;
    } else if has_sub_file(&path, "pom.xml") {
        return Lang::Java;
    } else if has_sub_file(&path, "go.mod") {
        return Lang::Go;
    }

    return Lang::None;
}
