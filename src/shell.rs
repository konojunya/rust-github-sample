use std::{process::Command, str::from_utf8};

pub fn get_current_origin() -> (String, String) {
    // call: git config --get remote.origin.url
    let out = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .unwrap();

    // ssh://git@github.com/owner/repo.git or https://github.com/owner/repo.git
    let origin = from_utf8(&out.stdout).unwrap();
    let mut v: Vec<&str> = origin.split("/").collect();
    let repo_with_suffix = v.pop().unwrap();
    let owner = v.pop().unwrap();

    let mut v: Vec<&str> = repo_with_suffix.split(".").collect();
    v.reverse();
    let repo = v.pop().unwrap();

    (owner.to_owned(), repo.to_owned())
}
