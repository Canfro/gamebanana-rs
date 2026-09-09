use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Wip<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn wip(&'a self) -> Wip<'a> {
        Wip { api: self }
    }
}
