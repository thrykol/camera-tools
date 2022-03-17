use anyhow::Result;

use crate::cli::{Opt, ReadOpt};

pub mod cli;
pub mod google;
pub mod jwt;

struct Settings {}

impl Settings {
    fn load_token() -> Result<String> {
        todo!("dbus call not yet implemented")
    }
}

#[derive(Debug)]
pub struct FetchSettings {
    pub jwt: String,
    pub collection: String,
}

impl TryFrom<Opt> for FetchSettings {
    type Error = anyhow::Error;

    fn try_from(opt: Opt) -> Result<Self, Self::Error> {
        match opt {
            Opt::Read(ReadOpt { collection, token }) => token.ok_or_else(|| ()).or_else(|_| Settings::load_token()).map(|jwt| Self {
                collection,
                jwt,
            }),
        }
    }
}
