# Git_CLI_RUST

This is a small cli built in rust designed to iterate over local repo branches and make basic actions on them.
For now: ["show last commit", "delete local branch"]

### HOW TO USE IT:

Download the repo, move to the directory and run `cargo build --release`.
The built app is now in `PATH_TO_YOUR_CURRENT_DIR/target/release`. You can move/copy the complete release directory if you need to or just call the app from any local git repo using the complete path from `/home` if you're on linux or from `C:\\` => `COMPLETE_PATH_TO_YOUR_CURRENT_DIR/target/release/git_handler` to manage your current directory git repo.
Help is provided from the cli using `?`
