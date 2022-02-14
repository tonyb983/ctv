use crate::config::PermColors;
#[cfg(not(windows))]
use libc::mode_t;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[derive(Clone, Copy)]
enum PermType {
    User,
    Group,
    Other,
}

/// TODO: mode_t will probably have to be extracted, wrapped, and made platform-agnostic. I can
///   use configuration based flags for the functions but they still receive the type `time_t`
///   from libc which doesn't seem to be available on windows. `masks`, `check` and `format` all
///   need to be changed.
impl PermType {
    #[cfg(not(windows))]
    fn masks(self) -> (mode_t, mode_t, mode_t) {
        use libc::{
            S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
        };
        match self {
            Self::User => (S_IRUSR, S_IWUSR, S_IXUSR),
            Self::Group => (S_IRGRP, S_IWGRP, S_IXGRP),
            Self::Other => (S_IROTH, S_IWOTH, S_IXOTH),
        }
    }

    #[cfg(not(windows))]
    fn check(self, mode: mode_t) -> (bool, bool, bool) {
        let (read, write, exec) = self.masks();
        (mode & read > 0, mode & write > 0, mode & exec > 0)
    }

    #[cfg(not(windows))]
    pub fn format(self, mode: mode_t, colors: &PermColors) -> String {
        /// TODO: Is there any reason these are local functions and not private member functions?
        fn else_dash(
            cond: bool,
            if_true: colored::ColoredString,
            dash_color: crate::config::Color,
        ) -> colored::ColoredString {
            if cond {
                if_true
            } else {
                dash_color.apply("-")
            }
        }

        /// TODO: Is there any reason these are local functions and not private member functions?
        fn format_rwx((r, w, x): (bool, bool, bool), colors: &PermColors) -> String {
            format!(
                "{}{}{}",
                else_dash(r, colors.read.apply("r"), colors.none),
                else_dash(w, colors.write.apply("w"), colors.none),
                else_dash(x, colors.execute.apply("x"), colors.none),
            )
        }

        format_rwx(self.check(mode), colors)
    }
}

/// TODO: No immediate errors on this function but we'll see
pub fn perms(metadata: &std::fs::Metadata, colors: &PermColors) -> String {
    let mode = metadata.permissions().mode() as mode_t;

    let user = PermType::User.format(mode, colors);
    let group = PermType::Group.format(mode, colors);
    let other = PermType::Other.format(mode, colors);

    [user, group, other].join("")
}
