use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Dict<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn dict(&'a self) -> Dict<'a> {
        Dict { api: self }
    }
}
