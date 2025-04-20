// Integration tests for SqlLiteConfig.
// Ensure your `SqlLiteConfig` struct exposes `connection` or provides a getter for stats.
// Replace `your_crate` with the name in Cargo.toml under `[package].name`.
use rusty_hooker::hook_types::HookTypes;
use rusty_hooker::sqllite::SqlLiteConfig;

#[test]
fn test_add_and_check_hook() -> Result<(), Box<dyn std::error::Error>> {
    let config = SqlLiteConfig::new(":memory:")?;
    // Initially not managed
    assert!(!config.is_hook_managed("hook1")?);
    // Add and check
    config.add_hook("hook1")?;
    assert!(config.is_hook_managed("hook1")?);
    // Inserting same hook again should be ignored
    config.add_hook("hook1")?;
    assert!(config.is_hook_managed("hook1")?);
    Ok(())
}

#[test]
fn test_repo_hook_management() -> Result<(), Box<dyn std::error::Error>> {
    let config = SqlLiteConfig::new(":memory:")?;
    let repo = "repo1";
    let hook_type = HookTypes::PreCommit;

    // No hooks known initially
    assert!(!config.check_if_new_hook_is_known(repo, &hook_type)?);

    // Add hook and link to repo
    config.add_hook("hook1")?;
    config.add_hook_to_repo("hook1", repo, &hook_type)?;
    assert!(config.check_if_new_hook_is_known(repo, &hook_type)?);

    // Name matching
    assert!(config.check_if_new_hook_is_same(repo, &hook_type, "hook1")?);
    // Different name returns false
    assert!(!config.check_if_new_hook_is_same(repo, &hook_type, "hook2")?);
    Ok(())
}

#[test]
fn test_run_count_paths() -> Result<(), Box<dyn std::error::Error>> {
    let config = SqlLiteConfig::new(":memory:")?;
    config.add_hook("hook1")?;

    // Successful run
    config.add_successful_run("hook1")?;
    // Failed run on existing hook
    config.add_failed_run("hook1")?;

    // Run counts on non-existing hook (should not error)
    config.add_successful_run("nonexistent")?;
    config.add_failed_run("nonexistent")?;
    Ok(())
}
