use std::fs::Metadata;
#[cfg(not(windows))]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[cfg(not(windows))]
pub fn group(metadata: &Metadata) -> String {
    let group = users::get_group_by_gid(metadata.gid());
    if let Some(g) = group {
        String::from(g.name().to_string_lossy())
    } else {
        String::from(" ")
    }
}

/// TODO
#[cfg(windows)]
pub fn group(metadata: &Metadata) -> String {
    String::new()
}

#[cfg(not(windows))]
pub fn user(metadata: &Metadata) -> String {
    let user = users::get_user_by_uid(metadata.uid());
    if let Some(u) = user {
        String::from(u.name().to_string_lossy())
    } else {
        String::from(" ")
    }
}

/// TODO
#[cfg(windows)]
pub fn user(metadata: &Metadata) -> String {
    String::new()
}
