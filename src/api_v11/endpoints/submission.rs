use crate::GamebananaApiV11;

#[derive(Debug, Clone, Copy)]
pub struct Submission<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn submission(&'a self) -> Submission<'a> {
        Submission { api: self }
    }
}
