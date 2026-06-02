# bigdoc

check if your dev environment matches what a project needs.

```
------------- [PORTS]-------------
[IN USE]     5000 is in use
[FREE]       5432 is free
[FREE]       3000 is free
------------- [TOOLS]-------------
[OK]         cargo        1.95.0
[OK]         go           1.26.3
[OUTDATED]   node         22.22.3    (required >=23.0.1)
[MISSING]    ruby                    (required >=3.0.0)
```

## what it does

You clone a repo. It has a `.devspec.toml` that says "needs node 20, python 3.11, docker 24, and ports 5432, 8080 and 3000 free". Instead of manually checking each one, run `bigdoc check` and see exactly which tools are outdated/missing and which ports are not free on your machine. Useful for CI pipelines too. Run it as a step and it'll fail the build if the environment is mismatched.

## install

```bash
git clone https://github.com/bigblender/bigdoc
cd bigdoc
cargo install --path .
```

## usage

```bash
# check your environment against the spec
bigdoc check

# check and get fix suggestions for outdated/missing tools
bigdoc check --fix

# create a .devspec.toml in the current directory
bigdoc init
```

## the .devspec.toml file

commit a `.devspec.toml` in your repo root:
**Example:**
```toml
[tools]
node = ">=20"
python = ">=3.11"
docker = ">=24"
go = ">=1.22"

[ports]
required = [5432, 3000, 8080]
```

supports standard semver constraints: `>=`, `>`, `=`, `<`, `<=`, `~`, `^`.

## supported tools

| category | tools |
|---|---|
| languages | python, python3, node, ruby, go, java, rustc, gcc, clang |
| package managers | pip, pip3, cargo, npm, yarn, pnpm, bun, gem |
| tools | git, docker, kubectl, terraform, make, cmake, curl, wget |
