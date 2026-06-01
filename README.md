# bigdoc

check if your dev environment matches what a project needs.

```
[OK]         node         22.1.0
[OUTDATED]   python       3.9.7        (required >=3.11)
[MISSING]    ruby                      (required >=3.0.0)
```

## what it does

You clone a repo. The project has a `.devspec.toml` that says "needs node 20, python 3.11, docker 24". Instead of manually checking each one, run `bigdoc check` and see exactly what's wrong on your machine. 
Useful for CI pipelines too. Run it as a step and it'll fail the build if the environment is mismatched.

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

# create a .devspec.toml in the current directory
bigdoc init
```

## the spec file

commit a `.devspec.toml` in your repo root:

```toml
[tools]
node = ">=20"
python = ">=3.11"
docker = ">=24"
go = ">=1.22"
```

supports standard semver constraints: `>=`, `>`, `=`, `<`, `<=`, `~`, `^`.

## supported tools

| category | tools |
|---|---|
| languages | python, python3, node, ruby, go, java, rustc, gcc, clang |
| package managers | pip, pip3, cargo, npm, yarn, pnpm, bun, gem |
| tools | git, docker, kubectl, terraform, make, cmake, curl, wget |

## exit codes

- `0`: all checks passed
- `1`: one or more tools are outdated, missing, or have an invalid spec