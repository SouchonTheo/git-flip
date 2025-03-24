# git-flip

`git-flip` is a CLI tool designed to help developers quickly switch between multiple Git accounts. It automates the process of updating SSH keys and Git configurations, making it easier to manage multiple identities for different projects or organizations.

## Features

- Automatically removes existing SSH keys and adds the appropriate one for the selected account.
- Updates Git `user.name` and `user.email` configurations based on the selected account.
- Reads account configurations from a TOML file located at `~/.config/git-flip/config.toml`.

## Requirements

- **Operating System**: Only supported on macOS for now.
- **Dependencies**: Ensure `ssh-add` and `git` are installed and available in your system's PATH.

## Usage

1. Create a configuration file at `~/.config/git-flip/config.toml` with your account details.
2. Run the CLI with the desired account name:

   ```bash
   git-flip <account_name>
   ```

## Example Configuration File

```toml
[accounts.personal]
ssh_key = "~/path/to/personal/key"
git_username = "Your Personal Name"
git_email = "personal@example.com"

[accounts.work]
ssh_key = "~/path/to/work/key"
git_username = "Your Work Name"
git_email = "work@example.com"
```

## Disclaimer

This tool is currently only supported on macOS. Support for other operating systems may be added in the future.
