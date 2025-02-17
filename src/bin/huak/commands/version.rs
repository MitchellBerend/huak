use std::{env, process::ExitCode};

use crate::errors::{CliError, CliResult};
use huak::{ops, project::Project};

/// Run the `version` command.
pub fn run() -> CliResult<()> {
    let cwd = env::current_dir()?;
    let project = match Project::from(cwd) {
        Ok(p) => p,
        Err(e) => return Err(CliError::new(e, ExitCode::FAILURE)),
    };

    let version = ops::version::get_project_version(&project)
        .map_err(|e| CliError::new(e, ExitCode::FAILURE))?;

    let name = project.config().project_name();

    println!("{name}-{version}");

    Ok(())
}
