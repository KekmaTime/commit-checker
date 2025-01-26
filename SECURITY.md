# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Considerations

1. **GitHub Token Security**

- Store your GitHub token securely in `~/.config/commit-checker/.env`
- Never commit your token to version control
- Use tokens with minimal required permissions (only needs repo access)

2. **Local Git Operations**

- The tool only modifies local git commits
- No automatic pushing to remote repositories
- Pre-commit hook only reads commit information

3. **Configuration Security**

- Configuration file is stored in user's home directory
- Permissions are set appropriately for Unix systems
- Environment variables are read securely

## Reporting a Vulnerability

If you discover a security vulnerability in commit-checker, please:

1. **Do Not** open a public GitHub issue
2. Send a description of the vulnerability to [22am014@sctce.ac.in](mailto:22am014@sctce.ac.in)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

You can expect:

- Acknowledgment within 48 hours
- Regular updates on the progress
- Credit in the security advisory (if desired)

## Known Limitations

1. Windows support for file permissions is limited
2. GitHub API rate limiting may affect functionality
3. Token validation is basic - ensure your token has appropriate scopes

## Best Practices

1. Regularly rotate your GitHub token
2. Keep the tool updated to the latest version
3. Review commit messages before using suggested backdated commits
