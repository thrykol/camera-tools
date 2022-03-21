use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fs-document")]
pub enum Opt {
    #[structopt(name = "fetch")]
    Read(ReadOpt),
    #[structopt(name = "ls")]
    Ls(LsOpt),
}

#[derive(StructOpt, Debug)]
pub struct ReadOpt {
    /// Firestore collection to read from
    pub collection: String,

    /// Firebase access token.  If unset, application will attempt to retrieve one from DBus.
    #[structopt(short = "t")]
    pub token: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct LsOpt {
    /// Cloud Storage bucket
    pub bucket: String,

    /// Object path prefix.  Must end with a forward slash
    #[structopt(short = "p")]
    pub prefix: String,

    /// Firebase access token.  If unset, application will attempt to retrieve one from DBus.
    #[structopt(short = "t")]
    pub token: Option<String>,
}