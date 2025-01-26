# Commit Checker

A Git pre-commit hook tool that ensures you maintain a minimum number of commits per day, with support for backdated commits. Perfect for maintaining consistent GitHub activity.

## Features

- 📅 Track daily commit requirements from a specified start date
- 🔧 Configurable minimum commits per day
- 🪝 Pre-commit hook integration
- ⏰ Generates correct backdated commit commands
- 🏠 Global configuration in user's home directory

## Installation

1. Clone the repository:

```bash
git clone https://github.com/KekmaTime/commit-checker.git
cd commit-checker
```

2. Install the binary:

```bash
cargo install --path .
```

## Configuration

Create a config file at `~/.config/commit-checker/.env` with:

```bash
GITHUB_TOKEN=your_github_token
GITHUB_USERNAME=your_github_username
START_DATE=from_what_day_you_nonstop_green_in_your_github_graph
REQUIRED_COMMITS=how_many_commits_you_need_to_make_per_day
```

Note: Your GitHub token needs repository access permissions.

## Usage

1. Install the pre-commit hook in your repository:

```bash
commit-checker install
```

2. Check your commit status:

```bash
commit-checker
```

3. If you have missing commits, the tool will show you the exact commands to use for backdated commits.

## How it Works

The tool:

1. Checks GitHub events for your commits using the GitHub API
2. Ensures minimum daily commits from START_DATE
3. Blocks regular commits if requirements aren't met
4. Provides exact commands for backdated commits with correct timestamps

## Security

- GitHub token is stored securely in your home directory
- Only modifies local git commits
- No automatic pushing to remote repositories
- See [SECURITY.md](SECURITY.md) for more details

## Contributing

Please read:

- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community guidelines
- [SECURITY.md](SECURITY.md) for reporting security issues

## License

MIT License - See [LICENSE](LICENSE) file
