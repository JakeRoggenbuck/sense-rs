use git::{has_git, is_local_git};

pub mod git;
pub mod lang;

fn main() {
    let git_dir = has_git("./".to_string());
    if git_dir {
        println!("Is git!");
    } else {
        println!("Is not git.");
    }

    let local = is_local_git("./".to_string());
    if local {
        println!("Is local!");
    } else {
        println!("Is not local.");
    }
}
