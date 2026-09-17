use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Auth<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn auth(&'a self) -> Auth<'a> {
        Auth { api: self }
    }
}
