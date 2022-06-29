use git::{has_git, is_local_git};
use lang::get_lang;

pub mod git;
pub mod lang;

fn main() {
    let language = get_lang("./".to_string());
    println!("{}", language);

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
