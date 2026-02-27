use std::collections::BTreeMap;

pub const MAIN_TEX_MAX_BYTES: usize = 1 * 1024 * 1024;
pub const MAX_FILES: usize = 64;
pub const MAX_TOTAL_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_PATH_LEN: usize = 256;
pub const MAX_FILE_BYTES: usize = 1 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidInput,
    InvalidUtf8,
    InvalidPath,
    PathTooLong,
    DuplicatePath,
    TooManyFiles,
    FileTooLarge,
    TotalBytesExceeded,
    MissingMainTex,
    InvalidMainTex,
}

#[derive(Default, Debug)]
pub struct Mount {
    files: BTreeMap<String, Vec<u8>>,
    total_bytes: usize,
    finalized: bool,
}

impl Mount {
    pub fn reset(&mut self) {
        self.files.clear();
        self.total_bytes = 0;
        self.finalized = false;
    }

    pub fn add_file(&mut self, path_bytes: &[u8], data: &[u8]) -> Result<(), Error> {
        if self.finalized {
            return Err(Error::InvalidInput);
        }
        if data.is_empty() {
            return Err(Error::InvalidInput);
        }
        if data.len() > MAX_FILE_BYTES {
            return Err(Error::FileTooLarge);
        }

        let path = normalize_path(path_bytes)?;

        if self.files.len() >= MAX_FILES {
            return Err(Error::TooManyFiles);
        }
        if self.files.contains_key(path) {
            return Err(Error::DuplicatePath);
        }

        let next_total = self
            .total_bytes
            .checked_add(data.len())
            .ok_or(Error::TotalBytesExceeded)?;
        if next_total > MAX_TOTAL_BYTES {
            return Err(Error::TotalBytesExceeded);
        }

        self.files.insert(path.to_owned(), data.to_vec());
        self.total_bytes = next_total;
        Ok(())
    }

    pub fn has_file(&self, path_bytes: &[u8]) -> Result<bool, Error> {
        let path = normalize_path(path_bytes)?;
        Ok(self.files.contains_key(path))
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized
    }

    pub fn finalize(&mut self) -> Result<(), Error> {
        if self.finalized {
            return Ok(());
        }
        if self.total_bytes > MAX_TOTAL_BYTES {
            return Err(Error::TotalBytesExceeded);
        }
        let main_tex = self.files.get("main.tex").ok_or(Error::MissingMainTex)?;
        validate_main_tex(main_tex).map_err(|_| Error::InvalidMainTex)?;
        self.finalized = true;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path).map(|bytes| bytes.as_slice())
    }
}

pub fn validate_main_tex(bytes: &[u8]) -> Result<(), Error> {
    if bytes.is_empty() || bytes.len() > MAIN_TEX_MAX_BYTES {
        return Err(Error::InvalidInput);
    }
    if bytes.iter().any(|byte| *byte == 0) {
        return Err(Error::InvalidInput);
    }
    let text = core::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
    if text.trim().is_empty() {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn normalize_path(path_bytes: &[u8]) -> Result<&str, Error> {
    if path_bytes.is_empty() {
        return Err(Error::InvalidInput);
    }
    if path_bytes.len() > MAX_PATH_LEN {
        return Err(Error::PathTooLong);
    }
    if path_bytes.iter().any(|byte| *byte == 0 || *byte == b'\\') {
        return Err(Error::InvalidPath);
    }

    let path = core::str::from_utf8(path_bytes).map_err(|_| Error::InvalidUtf8)?;
    if path.starts_with('/') {
        return Err(Error::InvalidPath);
    }

    let mut saw_segment = false;
    for segment in path.split('/') {
        if segment.is_empty() || segment == ".." {
            return Err(Error::InvalidPath);
        }
        saw_segment = true;
    }

    if !saw_segment {
        return Err(Error::InvalidPath);
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{validate_main_tex, Error, Mount, MAX_FILES, MAX_FILE_BYTES, MAX_PATH_LEN};

    fn valid_main() -> Vec<u8> {
        b"\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}\n".to_vec()
    }

    #[test]
    fn path_policy_rejects_invalid_paths() {
        let mut mount = Mount::default();
        let bytes = valid_main();

        let invalid_paths = [
            "/abs.tex",
            "../up.tex",
            "a/../b.tex",
            "a\\b.tex",
            "",
            "a//b.tex",
            "a/b/",
        ];

        for path in invalid_paths {
            let result = mount.add_file(path.as_bytes(), &bytes);
            assert!(result.is_err(), "expected path to fail: {path}");
        }
    }

    #[test]
    fn duplicate_path_rejected() {
        let mut mount = Mount::default();
        let bytes = valid_main();
        assert!(mount.add_file(b"dup.tex", &bytes).is_ok());
        assert_eq!(
            mount.add_file(b"dup.tex", &bytes),
            Err(Error::DuplicatePath)
        );
    }

    #[test]
    fn finalize_requires_main_tex() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"sub.tex", b"sub").is_ok());
        assert_eq!(mount.finalize(), Err(Error::MissingMainTex));
    }

    #[test]
    fn finalize_rejects_invalid_main_tex() {
        let mut mount = Mount::default();
        assert!(mount.add_file(b"main.tex", b" \n\t ").is_ok());
        assert_eq!(mount.finalize(), Err(Error::InvalidMainTex));
    }

    #[test]
    fn finalize_sets_finalized_and_blocks_additional_files() {
        let mut mount = Mount::default();
        let main = valid_main();
        assert!(mount.add_file(b"main.tex", &main).is_ok());
        assert!(mount.finalize().is_ok());
        assert!(mount.is_finalized());
        assert_eq!(mount.add_file(b"later.tex", b"x"), Err(Error::InvalidInput));
    }

    #[test]
    fn caps_enforced_for_file_size_and_path_len() {
        let mut mount = Mount::default();
        let oversize_file = vec![b'a'; MAX_FILE_BYTES + 1];
        assert_eq!(
            mount.add_file(b"big.tex", &oversize_file),
            Err(Error::FileTooLarge)
        );

        let long_path = vec![b'a'; MAX_PATH_LEN + 1];
        assert_eq!(
            mount.add_file(&long_path, b"x"),
            Err(Error::PathTooLong)
        );
    }

    #[test]
    fn caps_enforced_for_max_files() {
        let mut mount = Mount::default();
        for index in 0..MAX_FILES {
            let path = format!("f{index}.tex");
            assert!(mount.add_file(path.as_bytes(), b"x").is_ok());
        }
        assert_eq!(
            mount.add_file(b"overflow.tex", b"x"),
            Err(Error::TooManyFiles)
        );
    }

    #[test]
    fn has_file_and_finalize_success() {
        let mut mount = Mount::default();
        let main = valid_main();
        assert!(mount.add_file(b"main.tex", &main).is_ok());
        assert!(mount.add_file(b"sub.tex", b"sub").is_ok());

        assert_eq!(mount.has_file(b"main.tex"), Ok(true));
        assert_eq!(mount.has_file(b"missing.tex"), Ok(false));
        assert!(mount.finalize().is_ok());
        assert_eq!(mount.read_file("main.tex").unwrap(), main.as_slice());
    }

    #[test]
    fn validate_main_tex_checks_utf8_and_nul() {
        assert!(validate_main_tex(&valid_main()).is_ok());
        assert_eq!(validate_main_tex(&[0]), Err(Error::InvalidInput));
        assert_eq!(validate_main_tex(&[0xff]), Err(Error::InvalidUtf8));
    }
}

