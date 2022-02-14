use humansize::{file_size_opts as options, FileSize};
use std::fs::Metadata;
#[cfg(not(windows))]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[cfg(not(windows))]
fn get_unix(metadata: &Metadata) -> u64 {
    metadata.size()
}

#[cfg(windows)]
fn get_windows(metadata: &Metadata) -> u64 {
    metadata.file_size()
}

/// TODO: This seems to be all that is necessary for now, but i'll need to check if directories
///   get passed to this function, as `windows::fs::MetadataExt::file_size` only works for **files**
pub fn get(metadata: &Metadata) -> u64 {
    #[cfg(windows)]
    return get_windows(metadata);
    #[cfg(not(windows))]
    return get_unix(metadata);
}

pub fn format(size: u64) -> String {
    size.file_size(options::CONVENTIONAL).unwrap()
}
