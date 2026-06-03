# bigdoc

bigdoc is a dev environment drift detector. `bigdoc check` tells you exactly which tools are outdated, which are missing, and whether the ports your project needs are actually free, all checked against a .devspec.toml you commit alongside your code.

If no .devspec.toml file exists, run `bigdoc scan` and bigdoc reads your existing project files (package.json, Cargo.toml, go.mod, requirements.txt, pom.xml, docker-compose.yml, etc) to generate one automatically. Teams can also maintain a canonical spec remotely and pull it with `bigdoc sync`.

```
------------- [PORTS]-------------
[IN USE]     5000        is in use
[FREE]       5432        is free
[FREE]       3000        is free
------------- [TOOLS]-------------
[OK]         cargo       1.95.0
[OK]         go          1.26.3
[OUTDATED]   node        22.22.3     (required >=23.0.1)
[MISSING]    ruby                    (required >=3.0.0)

3 ok, 1 outdated, 1 missing
```

## installation

```bash
git clone https://github.com/bigblender2115/bigdoc
cd bigdoc
cargo install --path .
```

## usage

```bash
# check your environment against the spec
bigdoc check

# check and print fix suggestions for outdated or missing tools
bigdoc check --fix

# auto-generate a .devspec.toml from your project files
bigdoc scan

# pull a .devspec.toml from a remote URL
bigdoc sync https://raw.githubusercontent.com/yourorg/yourrepo/main/.devspec.toml

# create a default .devspec.toml in the current directory
bigdoc init
```

## the spec file

Commit a `.devspec.toml` in your repo root. Everyone on the team runs `bigdoc check` against it.

```toml
[tools]
node = ">=20"
python = ">=3.11"
docker = ">=24"
go = ">=1.22"
cargo = ">=1.70"

[ports]
required = [5432, 3000, 8080]
```

## auto-detection

Run `bigdoc scan` in your project root. bigdoc reads your project config files and generates a `.devspec.toml` automatically.

Supported files:

- `package.json`: node, npm, yarn, pnpm
- `Cargo.toml`: rustc, cargo
- `go.mod`: go
- `requirements.txt` / `pyproject.toml`: python, pip
- `Dockerfile`: docker
- `docker-compose.yml`: docker, plus auto-populates `[ports]` from port mappings
- `pom.xml`: java, maven
- `build.gradle` / `build.gradle.kts`: java

## supported tools

| category | tools |
|---|---|
| languages | python, python3, node, ruby, go, java, rustc, gcc, clang, deno, elixir, kotlin, scala, swift, php, lua, dotnet, zig |
| package managers | pip, pip3, cargo, npm, yarn, pnpm, bun, gem, composer, maven, gradle, poetry, pipenv, uv |
| tools | git, docker, kubectl, terraform, make, cmake, curl, wget, helm, ansible, vault, psql, mysql, sqlite3, ffmpeg, jq |

## built with

- [clap](https://github.com/clap-rs/clap): CLI
- [serde](https://serde.rs) + [toml](https://github.com/toml-rs/toml): spec parsing
- [semver](https://github.com/dtolnay/semver): version comparison
- [colored](https://github.com/mackwic/colored): terminal colors
- [regex](https://github.com/rust-lang/regex): version extraction
- [phf](https://github.com/rust-phf/rust-phf): compile-time tool map
- [reqwest](https://github.com/seanmonstar/reqwest): remote sync
