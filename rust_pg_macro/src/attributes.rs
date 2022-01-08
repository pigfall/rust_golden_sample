use darling::{FromMeta};

#[derive(FromMeta,Debug,Default)]
pub struct MainAttr{
    pub logger :Option<bool >,
}
