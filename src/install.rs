use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn install_pre_commit_hook() -> std::io::Result<()> {
    let git_dir = Path::new(".git");
    let hooks_dir = git_dir.join("hooks");
    let pre_commit = hooks_dir.join("pre-commit");

    let hook_content = r#"#!/bin/sh
# Check if commit requirements are met
echo "Checking commit requirements..."

# For regular commits, check requirements
commit-checker
if [ $? -ne 0 ]; then
    echo
    echo "❌ Cannot commit: You must use the backdated commit format"
    echo "Run 'commit-checker' to see the correct commit commands"
    echo
    exit 1
fi
echo "✓ Commit requirements met"
"#;

    fs::create_dir_all(&hooks_dir)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&pre_commit)?;

    file.write_all(hook_content.as_bytes())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&pre_commit)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit, perms)?;
    }

    Ok(())
}
