# Design

The CLI should have the following commands:
    - Scan?
    - List repos: List the current managed repos
    - List hooks: List the current hooks (maybe read form the config?)
    - Apply hook <hook> <repo>: Apply a given hook to a repo
    - Test: Check the current config
    - Run hook <hook>: Run the hook from a given directory (To update )

The config file, should contain all the hooks, a hooks should have the following properties:
    - The name
    - The path to the script or cmd?
    - The glob pattern, by the git diif (maybe file names or code language?)
    - Optional description

