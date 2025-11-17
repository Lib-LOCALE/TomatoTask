# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The TomatoTask team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

If you discover a security vulnerability, please report it by:

1. **Email**: Send details to snpepito@gmail.com
2. **Private Security Advisory**: Use GitHub's private vulnerability reporting feature at https://github.com/AnthonyMahe/TomatoTask/security/advisories/new

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., XSS, SQL injection, remote code execution)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit/direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability and how an attacker might exploit it

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt of your report within 48 hours
- **Updates**: We'll provide regular updates on the progress of addressing the vulnerability
- **Timeline**: We aim to resolve critical vulnerabilities within 90 days
- **Credit**: We'll credit you in the security advisory (unless you prefer to remain anonymous)

### Security Update Process

1. Security issue is reported and confirmed
2. A fix is prepared in a private repository
3. A new version is released with the fix
4. A security advisory is published
5. Users are notified to update

## Security Best Practices

When using TomatoTask:

- Always download the application from official sources (GitHub releases)
- Keep your application updated to the latest version
- Review permissions requested by the application
- Report suspicious behavior immediately

## Scope

The following are **in scope** for vulnerability reports:

- TomatoTask desktop application (Tauri)
- Frontend code (Svelte)
- Backend Rust code
- Build and release processes

The following are **out of scope**:

- Attacks requiring physical access to a user's device
- Social engineering attacks
- Denial of service attacks
- Issues in third-party dependencies (report these to the respective projects)

## Safe Harbor

We support safe harbor for security researchers who:

- Make a good faith effort to avoid privacy violations and disruptions
- Only interact with accounts you own or with explicit permission
- Do not exploit vulnerabilities beyond the minimum necessary to demonstrate the issue
- Report vulnerabilities promptly
- Keep vulnerability details confidential until we've addressed them

We will not pursue legal action against researchers who follow these guidelines.

## Contact

For any security-related questions or concerns, please contact the maintainers at snpepito@gmail.com.

---

Thank you for helping keep TomatoTask and its users safe!
