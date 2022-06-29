# sense-py
Sense the directories around you, are they git repositories, what language, etc.

## Languages
[ :snake: sense-py](https://github.com/JakeRoggenbuck/sense-py) || [:crab: sense-rs](https://github.com/JakeRoggenbuck/sense-rs) || 
[:hamster: sense-go](https://github.com/JakeRoggenbuck/sense-go) || [🇨 sense-c](https://github.com/JakeRoggenbuck/sense-c)

## Why?
#### Why so many langs?
Because I write projects pretty regularly in all of these languages and want a consistent API and available library support for all of them.

## API

### Git

```rs
has_git(path: String) -> bool;
is_local_git(path: String) -> bool;
```

### Language
```rs
get_lang(path: String) -> Lang;
```

## Example
```rs
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
```

## Supported langs
- [x] Python
- [x] JavaScript
- [x] Rust
- [x] Java
- [x] Go
- [ ] TypeScript
- [ ] C
- [ ] CPP

## Contributing
If you would like to add features or language support, that would be amazing!
