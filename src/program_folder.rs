#[derive(Eq)]
pub struct ProgramFolder {
    pub bytes: u64,
    pub path: String,
}

impl Ord for ProgramFolder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.bytes > other.bytes {
            return std::cmp::Ordering::Greater;
        }
        if self.bytes < other.bytes {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for ProgramFolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.bytes.partial_cmp(&other.bytes) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.path.partial_cmp(&other.path)
    }
}

impl PartialEq for ProgramFolder {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes && self.path == other.path
    }
}