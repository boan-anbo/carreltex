#[derive(Default)]
pub(super) struct OkTitleStateV0 {
    pub(super) title: Option<Vec<u8>>,
    pub(super) author: Option<Vec<u8>>,
    pub(super) date: Option<Vec<u8>>,
}

impl OkTitleStateV0 {
    pub(super) fn set_field(&mut self, name: &[u8], value: Vec<u8>) {
        let normalized = if value.is_empty() { None } else { Some(value) };
        match name {
            b"title" => self.title = normalized,
            b"author" => self.author = normalized,
            b"date" => self.date = normalized,
            _ => {}
        }
    }
}
