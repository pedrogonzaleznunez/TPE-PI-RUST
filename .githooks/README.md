# Git Hooks

This directory contains Git hooks that enforce code quality and commit message standards for this project.

## Conventional Commits Hook

The `commit-msg` hook enforces the [Conventional Commits](https://www.conventionalcommits.org/) specification for all commit messages.

### Setup

To enable the Git hooks, run the setup script:

```bash
./githooks/setup.sh
```

This will configure Git to use the hooks in this directory.

### Bypassing the Hook

In rare cases where you need to bypass the hook (not recommended), you can use:

```bash
git commit --no-verify -m "emergency fix"
```

### Disabling the Hook

To disable the hook, reset Git's hooks path:

```bash
git config --unset core.hooksPath
```

## Troubleshooting

If the hook isn't working:

1. Ensure the hook file is executable: `chmod +x githooks/commit-msg`
2. Check that Git is configured to use the hooks directory: `git config core.hooksPath`
3. Verify you're in a Git repository: `git status`
