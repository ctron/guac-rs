use crate::query::QueryConfig;
use colored_json::ToColoredJson;
use guac::client::GuacClient;
use packageurl::PackageUrl;
use std::process::ExitCode;
use std::str::FromStr;

#[derive(clap::Args, Debug)]
#[command(
    about = "Run the query to find all dependencies of the package (purl)",
    args_conflicts_with_subcommands = true
)]
pub struct DependenciesCommand {
    #[command(flatten)]
    pub(crate) config: QueryConfig,
}

impl DependenciesCommand {
    pub async fn run(self) -> anyhow::Result<ExitCode> {
        let guac = GuacClient::new(&self.config.guac_url);
        let purl = PackageUrl::from_str(&self.config.purl)?;
        let deps = guac.semantic().dependents_of(&purl).await?;
        let out =
            serde_json::to_string(&deps)?.to_colored_json(crate::color_mode(self.config.color))?;
        println!("{}", out);

        Ok(ExitCode::SUCCESS)
    }
}
