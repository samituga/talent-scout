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

## Cargo Dependencies

#### sqlx

<details>

#### 1. Install sqlx

```bash
cargo install sqlx-cli
```

#### 2. Prepare offline environment

Generate offline sql data (needs postgres to be running)\
sqlx calls into our database at compile-time to ensure that all queries can be successfully executed considering
the schemas of our tables.\
We use this command locally to save the results so that we don't need a live connection on the CI pipelines.\
We will need the env variable `SQLX_OFFLINE=true` to use the offline data

```bash
cargo sqlx prepare --workspace -- --all-targets
```

We can also use it locally

```bash
SQLX_OFFLINE=true cargo run
```

</details>

## Local environment

Initialize the DB by executing the script

```bash
./scripts/compose-up.sh
```

It will execute the `docker compose up`, set up the app user and db and execute the DB migrations