use anyhow::{Result, anyhow};
use clap::Args;

use crate::config::DistributeOptions;

#[derive(Args)]
pub struct ReleaseArgs {
    /// Name of the release to run (matches a `name:` key in
    /// `distribute_options.yaml`). Required.
    #[arg(long = "name", value_name = "NAME")]
    pub name: Option<String>,

    /// Comma-separated list of job names to run.
    /// When specified, only these jobs are executed.
    #[arg(long = "jobs", value_name = "JOB,...")]
    pub jobs: Option<String>,

    /// Comma-separated list of job names to skip.
    /// Ignored when `--jobs` is also provided.
    #[arg(long = "skip-jobs", value_name = "JOB,...")]
    pub skip_jobs: Option<String>,

    /// Skip `flutter clean` before packaging.
    #[arg(long = "skip-clean", default_value_t = false)]
    pub skip_clean: bool,

    /// Perform a dry run: print which jobs would execute without actually
    /// running them.
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
}

pub async fn execute(args: &ReleaseArgs) -> Result<()> {
    let release_name = args
        .name
        .as_deref()
        .ok_or_else(|| anyhow!("The 'name' option is mandatory!"))?;

    let job_names: Vec<String> = args
        .jobs
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let skip_job_names: Vec<String> = args
        .skip_jobs
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let opts = DistributeOptions::load()?;

    // Resolve global variables: env vars < distribute_options.variables
    let mut global_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    global_vars.extend(opts.resolved_variables());

    // Select matching releases
    let matching: Vec<_> = opts
        .releases
        .iter()
        .filter(|r| r.name == release_name)
        .collect();

    if matching.is_empty() {
        return Err(anyhow!(
            "No release named '{}' found in distribute_options.yaml.",
            release_name
        ));
    }

    for release in &matching {
        let filtered_jobs = release.filter_jobs(&job_names, &skip_job_names);

        if filtered_jobs.is_empty() {
            return Err(anyhow!(
                "No available jobs found in release '{}'.",
                release.name
            ));
        }

        for job in &filtered_jobs {
            // Merge variables: global < release-level < job-level
            let mut _vars = global_vars.clone();
            if let Some(rv) = &release.variables {
                _vars.extend(rv.clone());
            }
            if let Some(jv) = &job.variables {
                _vars.extend(jv.clone());
            }

            log::info!(
                "===> Releasing {}:{} (platform={}, target={})",
                release.name,
                job.name,
                job.package.platform,
                job.package.target,
            );

            if args.dry_run {
                println!(
                    "[dry-run] Would package {}:{} ({}/{})",
                    release.name,
                    job.name,
                    job.package.platform,
                    job.package.target,
                );
                if let Some(target) = job.publish_target() {
                    println!(
                        "[dry-run] Would publish {}:{} to {}",
                        release.name, job.name, target,
                    );
                }
                continue;
            }

            // TODO(M5): delegate to app_builder + app_packager + app_publisher
            // once those crates expose stable async interfaces.
            // For now we print the resolved job plan so the output is testable.
            println!(
                "Release job '{}:{}': package {}/{} (clean={})",
                release.name,
                job.name,
                job.package.platform,
                job.package.target,
                !args.skip_clean,
            );
            if let Some(target) = job.publish_target() {
                println!(
                    "Release job '{}:{}': publish to {}",
                    release.name, job.name, target,
                );
            }
            let _ = _vars;
        }
    }

    Ok(())
}
