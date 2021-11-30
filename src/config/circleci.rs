use super::{Config, Env, Result};

pub(super) fn load_config(config: &mut Config, env: &Env) -> Result<()> {
    if let Some(v) = env.get_var("CIRCLE_BRANCH")? {
        config.git_branch.replace(v);
    }

    if let Some(v) = env.get_var("CIRCLE_WORKFLOW_JOB_ID")? {
        config.service_job_id.replace(v);
    }

    if let Some(v) = env.get_var("CIRCLE_WORKFLOW_ID")? {
        config.service_number.replace(v);
    }

    if let Some(v) = env.get_var("CIRCLE_BUILD_NUM")? {
        config.service_number.replace(v);
    }

    if let Some(v) = env.get_var("CIRCLE_PULL_REQUEST")? {
        config.service_pull_request.replace(v.split("/").last().unwrap_or(&v).to_string());
    }

    if let Some(v) = env.get_var("CIRCLE_BUILD_URL")? {
        config.service_build_url.replace(v);
    }

    Ok(())
}
