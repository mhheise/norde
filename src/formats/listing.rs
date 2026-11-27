//! Color choices for file listings, shared by eza and lsd so the two agree.
//!
//! File names and permissions use syntax hues; secondary metadata (dates,
//! inodes, punctuation) is muted.

use crate::color::Color;
use crate::theme::Variant;

pub struct Listing {
    pub text: Color,
    pub muted: Color,
    pub error: Color,

    pub directory: Color,
    pub executable: Color,
    pub symlink: Color,
    /// Pipes, sockets, and devices.
    pub special: Color,

    pub read: Color,
    pub write: Color,
    pub exec: Color,
    /// Setuid, setgid, and sticky bits.
    pub special_perm: Color,

    pub size_small: Color,
    pub size_medium: Color,
    pub size_large: Color,

    pub you: Color,

    pub added: Color,
    pub modified: Color,
    pub deleted: Color,
    pub renamed: Color,
    pub ignored: Color,
    pub conflicted: Color,

    /// Images, video, and music.
    pub media: Color,
    pub archive: Color,
    pub crypto: Color,
    /// Build files, such as `Makefile` or `Cargo.toml`.
    pub build: Color,
}

impl Listing {
    pub fn new(v: &Variant) -> Listing {
        let (x, vcs) = (&v.syntax, &v.ui.vcs);
        Listing {
            text: v.ui.text.default,
            muted: x.comment.color,
            error: v.ui.status.error,

            directory: x.keyword.color,
            executable: x.ty.color,
            symlink: x.constant.color,
            special: x.function.color,

            read: x.function.color,
            write: x.string.color,
            exec: x.ty.color,
            special_perm: x.constant.color,

            size_small: x.ty.color,
            size_medium: x.function.color,
            size_large: x.string.color,

            you: x.function.color,

            added: vcs.added,
            modified: vcs.modified,
            deleted: vcs.deleted,
            renamed: vcs.renamed,
            ignored: vcs.ignored,
            conflicted: vcs.conflict,

            media: x.constant.color,
            archive: x.string.color,
            crypto: x.string_special.color,
            build: x.function.color,
        }
    }
}
