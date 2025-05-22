# Crab Hooks
A Rust-Based CLI program to manage and run git hooks

# Installation

```bash
cargo install crab-hooks
```

# Usage
For now it supports adding the hooks in a yaml file in $HOME_DIR/.config/crab_hooks.yml, alternatively the path to a config file might be passed along as --config-file <CONFIG_FILE_PATH>

Please see the current config in the root folder of the project, for an example of how to set it up.

After adding the hooks to the config, run `crab-hooks add HOOK_NAME`, where HOOK_NAME is the name given in the config file. This must be done inside the git repo you wish to manage the hooks in.

## Commands

Currently, crab-hooks supports the following commands:
 - test: Test the config file for validity, and if all commands resolve to a valid command - does not actively execute anything.
 - run: run a GIT_HOOK, actively used by the apply-hook command
 - list-hooks: List the current hooks in the config file
 - apply-hook: <GIT_HOOK> <HOOK_TYPE> apply a git hook from the config as the hook type in the current repo - does not work on unmanaged hook types.
 - remove-hook: Remove the GIT_HOOK as a hook type from the current repo.
 - delete-hook: Delete the hook from the repo.
 - help: Displays a help message.
