# uvenv Snap Package

uvenv is a simple command-line tool for managing virtual environments, written in Rust. Think of it as pipx, but for uv.

## Getting started

  ```bash
  snap install uvenv
  ```

## Snap Installation Caveats

When installed via Snap, there are some important differences to note:

- Tools are downloaded to `~/snap/uvenv/<revision>/.local/uvenv` instead of `~/.local/uvenv`
- Scripts are installed in `~/snap/uvenv/<revision>/.local/bin` instead of `~/.local/bin`
- The snap package cannot update files like `~/.bashrc` or perform self-updates.

## Setting Up Bash Integration

To enable all Bash-specific features, add the following lines to your `~/.bashrc`:

```bash
eval "$(uvenv --generate=bash ensurepath)" # Required: Fix PATH
eval "$(uvenv --generate=bash completions)" # Optional: Enable tab completion
eval "$(uvenv --generate=bash activate _)" # Optional: Enable the `uvenv activate` command
```

For other shells, run:

```bash
uvenv setup
```

This will display the appropriate setup instructions for your shell.

