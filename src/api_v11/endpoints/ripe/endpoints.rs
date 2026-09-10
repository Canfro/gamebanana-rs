use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Ripe<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn ripe(&'a self) -> Ripe<'a> {
        Ripe { api: self }
    }
}
