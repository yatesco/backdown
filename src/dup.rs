use {
    lazy_regex::*,
    std::path::{Path, PathBuf},
};

// TODO virer et utiliser PathBuf directement ?
#[derive(Debug)]
pub struct DupFile {
    pub path: PathBuf,
    // pub staged_for_removal: bool,
}

/// the list of files having a hash
/// TODO rename DupSet ?
#[derive(Debug, Default)]
pub struct DupSet {
    pub files: Vec<DupFile>, // identical files
    pub file_len: u64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DupFileRef {
    pub dup_set_idx: usize,
    pub dup_file_idx: usize,
}

impl DupFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            //staged_for_removal: false,
        }
    }
}

impl DupFileRef {
    pub fn path(self, dups: &[DupSet]) -> &Path {
        &dups[self.dup_set_idx].files[self.dup_file_idx].path
    }
    pub fn file_name(self, dups: &[DupSet]) -> String {
        self.path(dups)
            .file_name()
            .map_or_else(|| "".to_string(), |n| n.to_string_lossy().to_string())
    }
    /// get the file name when the file has a name like "thing (3).jpg"
    /// or "thing (3rd copy).png"
    pub fn copy_name(self, dups: &[DupSet]) -> Option<&str> {
        copy_name(self.path(dups))
    }
    /// tells whether the file has a name like "thing (3).jpg"
    /// or "thing (3rd copy).png"
    pub fn is_copy_named(self, dups: &[DupSet]) -> bool {
        self.copy_name(dups).is_some()
    }
}

/// get the name if this path is of a "copy" file, that is an usual name for a copy
pub fn copy_name(path: &Path) -> Option<&str> {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .filter(|n| {
            regex_is_match!(
                r#"(?x)
            .+
            \((
                \d+
            |
                [^)]*
                copy
            )\)
            (\.\w+)?
            $
        "#,
                n
            )
        })
}

#[test]
fn test_is_copy_named() {
    use std::path::PathBuf;
    let copies = &[
        "/some/path/to/bla (3).jpg",
        "bla (3455).jpg",
        "uuuuu (copy).rs",
        "/home/dys/Images/pink hexapodes (another copy).jpeg",
        "~/uuuuu (copy)",
        "uuuuu (3rd copy)",
    ];
    for s in copies {
        assert!(copy_name(&PathBuf::from(s)).is_some());
    }
    let not_copies = &[
        "copy",
        "copy.txt",
        "bla.png",
        "/home/dys/not a copy",
        "(don't copy)",
    ];
    for s in not_copies {
        assert!(copy_name(&PathBuf::from(s)).is_none());
    }
}

#[cfg(test)]
mod qc_tests {
    use std::str::FromStr;

    use super::*;
    use ::quickcheck::Arbitrary;

    quickcheck! {
    fn qc_is_copy_named_for_copy_paths(wrapper: WrapperForCopy) -> bool{
        copy_name(&wrapper.path).is_some()
    }
    }

    quickcheck! {
    fn qc_is_not_copy_named_for_not_copy_paths(wrapper: WrapperForNotACopy) -> bool{
        copy_name(&wrapper.path).is_none()
    }
    }

    #[derive(Clone, Debug)]
    struct WrapperForCopy {
        path: PathBuf,
    }
    #[derive(Clone, Debug)]
    struct WrapperForNotACopy {
        path: PathBuf,
    }

    impl Arbitrary for WrapperForCopy {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let copies = &[
                "/some/path/to/bla (3).jpg",
                "bla (3455).jpg",
                "uuuuu (copy).rs",
                "/home/dys/Images/pink hexapodes (another copy).jpeg",
                "~/uuuuu (copy)",
                "uuuuu (3rd copy)",
            ];
            let path = generate_string(g, copies);
            Self { path }
        }
    }
    impl Arbitrary for WrapperForNotACopy {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let not_copies = &[
                "copy",
                "copy.txt",
                "bla.png",
                "/home/dys/not a copy",
                "(don't copy)",
            ];

            let path = generate_string(g, not_copies);
            Self { path }
        }
    }

    fn generate_string(g: &mut quickcheck::Gen, candidates: &[&str]) -> PathBuf {
        // TODO - loop to ensure the generated pathbuf doesn't already contain inappropriate
        // text
        let candidate: &str = candidates[usize::arbitrary(g) % candidates.len()];
        let s = PathBuf::arbitrary(g);
        let os_string = s.into_os_string().into_string().unwrap();
        let os_string = format!("{}{}", os_string, candidate);
        PathBuf::from_str(&os_string).unwrap()
    }
}
