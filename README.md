# Crab Hooks
A Rust-Based CLI program to manage and run git hooks

# Installation

```bash
git clone https://github.com/jaller698/crab_hooks
cd crab_hooks
cargo install --path .
```

# Usage
For now it supports adding the hooks in a yaml file in $HOME_DIR/.config/crab_hooks.yml

Please see the current config in the root folder of the project, for an example of how to set it up.

After adding the hooks to the config, run `crab_hooks add HOOK_NAME`, where HOOK_NAME is the name given in the config file. This must be done inside the git repo you wish to manage the hooks in.

