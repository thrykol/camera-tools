use structopt::StructOpt;

use vivint_tools::{FetchSettings, LsBucketSettings};
use vivint_tools::cli::Opt;
use vivint_tools::google::{Firestore, Storage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Read(..) => {
            let settings = FetchSettings::try_from(opt)?;
            let _ = Firestore::get_document(&settings.collection, &settings.jwt).await?;
        }
        Opt::Ls(..) => {
            let settings = LsBucketSettings::try_from(opt)?;
            let _ = Storage::ls(&settings.bucket, &settings.prefix, &settings.jwt).await?;
        }
    }

    Ok(())
}
