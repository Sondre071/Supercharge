#[allow(non_snake_case, dead_code)]
pub struct Actions {
    pub ClearLine: &'static str,
}

pub const ACTIONS: Actions = Actions {
    ClearLine: "\x1b[2K",
};
