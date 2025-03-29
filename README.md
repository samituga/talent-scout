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

#### cargo sort

<details>

Sorts dependencies alphabetically

#### 1. Install cargo-sort

```bash
cargo install cargo-sort
```

#### 2. Sort dependencies for all members

```bash
cargo sort -w
```

</details>

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

#### 3. Add migration

Needs the environment variable DATABASE_URL to be set

```bash
sqlx migrate add <migration_name>
```

We can set the env variable inline

```bash
DATABASE_URL=postgres://user:pass@localhost:5432/talent-scout sqlx migrate add <migration_name>
```

#### 4. Start migration

```bash
sqlx migrate run
```

</details>

#### sea orm

<details>

#### 1. Install sea orm

```bash
cargo install sea-orm-cli
```

#### 2. Generate entities

Needs the environment variable DATABASE_URL to be set

```bash
sea-orm-cli generate entity -s <schema_name> -o <output_folder>
```

We can set the env variable inline

```bash
DATABASE_URL=postgres://user:pass@localhost:5432/talent-scout sea-orm-cli generate entity -s <schema_name> -o <output_folder>
```

</details>

## Local environment

Initialize the DB by executing the script

```bash
./scripts/compose-up.sh
```

If it's necessary to reset the local db there is a script for that. \
It needs to be executed in the project root directory. \
It will stop and delete the docker container, delete the postgres volume and execute the `compose-up.sh` script

```bash
./scripts/hard_reset_db.sh
```

It will execute the `docker compose up`, set up the app user and db and execute the DB migrations

#### Notes

`cargo-sort 1.0.9` does not support `key.workspace = true` instead use `key = { workspace = true }` \

Migrations for the riot models were created with assistance of the generated postgres, using `openapi-generator-cli` on a schema from a 3rd party openapi of the riot api.
It doesn't support relationships so that works is manual for now at least

```bash
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://www.mingweisamuel.com/riotapi-schema/openapi-3.0.0.yml \
    -g postgresql-schema \
    -o /local/out/sql
```
