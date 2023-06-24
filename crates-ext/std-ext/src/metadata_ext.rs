pub trait MetadataExt {
    fn mode_symbolic(&self) -> String;
}

impl<T: std::os::unix::fs::MetadataExt> MetadataExt for T {
    #[allow(clippy::identity_op)]
    fn mode_symbolic(&self) -> String {
        let mode_numeric = self.mode();
        let mode_strings = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

        format!(
            "{}{}{}",
            mode_strings[(mode_numeric as usize >> 6) & 0b111],
            mode_strings[(mode_numeric as usize >> 3) & 0b111],
            mode_strings[(mode_numeric as usize >> 0) & 0b111],
        )
    }
}
