extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate toml;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "advisory-db-renderer")]
struct Opt {
    #[structopt(name = "ADVISORY_DB_DIR", parse(from_os_str))]
    advisory_db_dir_path: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Advisories {
    advisory: Option<Vec<Advisory>>,
}

#[derive(Deserialize, Debug)]
struct Advisory {
    id: String,
    package: String,
    unaffected_versions: Option<Vec<String>>,
    patched_versionos: Option<Vec<String>>,
    dwf: Vec<String>,
    url: String,
    title: String,
    date: String,
    description: String,
}

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let advisories_toml_path = opt.advisory_db_dir_path.join("Advisories.toml");

    if !advisories_toml_path.exists() {
        return Err(
            format!(
                "Could not find Advisories.toml in {}",
                opt.advisory_db_dir_path.display()
            ).into()
        );
    }

    let advisories_toml_content = fs::read_to_string(advisories_toml_path)?;

    let advisories: Advisories = toml::from_str(&advisories_toml_content)?;

    println!("{:?}", advisories);

    Ok(())
}
