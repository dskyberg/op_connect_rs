# Rust 1Password Connect SDK
Fetch vaults and items from [1Password Connect](https://developer.1password.com/docs/connect).

This crate is designed to simply embed secrets mgt into Rust apps.  It supports both
blocking or async mode (default).

Field values are zeroized on drop.  If you clone them, you own them.

# OP_API_TOKEN
You must set your 1Password API token in the env.