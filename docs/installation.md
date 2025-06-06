# Advanced Installation Options

Explore multiple alternative ways to install `uvenv` on systems where global pip installs are restricted (e.g., Ubuntu 24.04+).
Each method offers a different approach, with its own benefits and setup steps.

---

## 1. via `install.sh`

The easiest way to install `uvenv` is to use the [`install.sh`](https://github.com/robinvandernoord/uvenv/blob/uvenv/install.sh) script.

**Advantages:**

* One-liner installation.
* Automatically fetches and installs the latest version.
* Compatible with various shells (`bash`, `sh`, `zsh`, etc.).

**Considerations:**

* Executes a remote script directly; review it if you have security concerns.

**Installation Steps:**

```bash
# download/read the script:
curl -fsSL https://raw.githubusercontent.com/robinvandernoord/uvenv/uvenv/install.sh

# run it:
bash -c "$(curl -fsSL https://raw.githubusercontent.com/robinvandernoord/uvenv/uvenv/install.sh)"
# instead of `bash`, you can also use `sh`, `zsh`, or "$SHELL"
```

---

## 2. via uv

If you already have [`uv`](https://github.com/astral-sh/uv) installed, you can use it to install `uvenv` as a managed tool.

**Advantages:**

* Isolated tool management via `uv`.
* Simplifies updates and uninstalls.
* No impact on system Python packages.

**Considerations:**

* Requires `uv` to be installed beforehand.

**Installation Steps:**

```bash
uv tool install uvenv
```

---

## 3. System Package Method

Install `uvenv` using `pip` with the `--break-system-packages` flag.

**Advantages:**

* Quick, no extra tooling required.
* Easy to use on minimal systems.

**Considerations:**

* Minor risk of package conflicts, though unlikely with `uvenv`.

**Installation Steps:**

```bash
pip install --break-system-packages uvenv
```

---

## 4. Pipx Installation Method

Use `pipx` to manage `uvenv` in an isolated environment.

**Advantages:**

* Keeps `uvenv` isolated from system packages.
* Easily updatable and removable.

**Considerations:**

* Requires `pipx` to be installed (`sudo apt install pipx` or equivalent).

**Installation Steps:**

```bash
pipx install uvenv
```

---

## 5. Virtual Environment Method

Create a dedicated Python virtual environment and install `uvenv` inside it.

**Advantages:**

* Complete isolation from system Python.
* Suitable for users comfortable with virtual environments.

**Considerations:**

* Requires familiarity with `venv` and virtual environments.
* Needs activation each time or linking via `uvenv self link`.

**Installation Steps:**

```bash
python -m venv ~/.virtualenvs/uvenv
source ~/.virtualenvs/uvenv/bin/activate
pip install uvenv
uvenv self link  # or `uvenv setup` for full integration
```

---

## 6. Self-Managed uvenv Method

Use `uvenv` to manage and update its own installation.

**Advantages:**

* Allows `uvenv` to bootstrap and maintain itself.
* Streamlines long-term tool management.

**Considerations:**

* Requires initial manual setup.
* Commands like `uvenv uninstall-all` may remove itself—use with care.

**Installation Steps:**

```bash
python -m venv /tmp/initial-uvenv
source /tmp/initial-uvenv/bin/activate
pip install uvenv
uvenv install uvenv
uvenv ensurepath  # or `uvenv setup` for all features
```

---

## 7. via Snap

Snap installation is also supported.

**Advantages:**

* Easy to install and manage on Snap-enabled systems.
* Clean separation from system packages.

**Considerations:**

* Snap-specific behavior and confinement apply.
* See [snap installation](./snap.md) for full instructions and caveats.
