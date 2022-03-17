use structopt::StructOpt;

use vivint_tools::cli::Opt;
use vivint_tools::FetchSettings;
use vivint_tools::google::Firestore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Read(..) => {
            let settings = FetchSettings::try_from(opt)?;
            let _ = Firestore::get_document(&settings.collection, &settings.jwt).await?;
        }
    }

    Ok(())
}
