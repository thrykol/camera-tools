use anyhow::Result;

use crate::cli::{DownloadOpt, LsOpt, Opt, ReadOpt};

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
            _ => unimplemented!("command {:#?} not support in fetch", opt),
        }
    }
}

#[derive(Debug)]
pub struct LsBucketSettings {
    pub bucket: String,
    pub prefix: String,
    pub jwt: String,
}

impl TryFrom<Opt> for LsBucketSettings {
    type Error = anyhow::Error;

    fn try_from(opt: Opt) -> Result<Self, Self::Error> {
        match opt {
            Opt::Ls(LsOpt { bucket, prefix, token }) => token.ok_or_else(|| ()).or_else(|_| Settings::load_token()).map(|jwt| Self {
                bucket,
                prefix,
                jwt,
            }),
            _ => unimplemented!("command {:#?} not support in ls", opt),
        }
    }
}

pub struct FetchObjectSettings {
    pub bucket: String,
    pub path: String,
    pub jwt: String,
    pub destination: String,
}


impl TryFrom<Opt> for FetchObjectSettings {
    type Error = anyhow::Error;

    fn try_from(opt: Opt) -> Result<Self, Self::Error> {
        match opt {
            Opt::Download(DownloadOpt { bucket, path, token, destination }) => token.ok_or_else(|| ()).or_else(|_| Settings::load_token()).map(|jwt| Self {
                bucket,
                path,
                jwt,
                destination,
            }),
            _ => unimplemented!("command {:#?} not support in ls", opt),
        }
    }
}