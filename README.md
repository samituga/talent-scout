# Talent Scout

description TBD

## Prerequisites

TBD

## Dependencies

#### Pre Commit

<details>

`pre-commit` is used to automatically run quality checks, such as formatting and linting, before each commit.

#### 1. Install pre-commit

Make sure you have Python and pip installed, then run:

```bash
pip install pre-commit
```

#### 2. Install the git hook scripts

```bash
pre-commit install
```

#### 3. (optional) Run against all the files ¶

- run pre-commit install to set up the git hook scripts
- (runs automatically before each commit but only for changed files)
```bash
pre-commit run --all-files
```

</details>