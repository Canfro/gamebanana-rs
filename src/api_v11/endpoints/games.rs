use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Games<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn games(&'a self) -> Games<'a> {
        Games { api: self }
    }
}
