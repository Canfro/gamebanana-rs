use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Wip<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn wip(&'a self) -> Wip<'a> {
        Wip { api: self }
    }
}
