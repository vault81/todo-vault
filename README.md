# TodoVault

[![Build & Test](https://github.com/vault81/todo-vault/actions/workflows/build.yml/badge.svg)](https://github.com/vault81/todo-vault/actions/workflows/build.yml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/vault81/todo-vault)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/vault81/todo-vault)](https://rust-reportcard.xuri.me/report/github.com/vault81/todo-vault)
[![license-badge](https://img.shields.io/github/license/vault81/todo-vault)](https://github.com/vault81/todo-vault/blob/main/LICENSE.md)

## Overview

TodoVault is a free and open-source web application designed to manage your to-do lists and facilitate collaboration with others.

### Stack

The current stack includes (but is not limited too):

- DB: [sqlite3](https://sqlite.org/index.html)
- ORM: [SeaORM](https://www.sea-ql.org/SeaORM/)
- Server Framework: [Axum](https://github.com/tokio-rs/axum)
- Web App Framework(with hydration+SSR): [leptos](https://github.com/leptos-rs/leptos)
- CSS Framework [TailwindCSS](Web App Framework(with hydration+SSR): [leptos](https://github.com/leptos-rs/leptos))

### Server System Requirements

- A somewhat recent linux kernel
- glibc

These only apply to default build settings.
You can compile the c runtime statically (take a look at .cargo/docker-config.toml), to get rid of the runtime dedependency on it.
Older systems or different operating systems and architectures may work if you compile yourself and play around with the settings in .cargo/config.toml & Cargo.toml.

## Development

### Building the App

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

Then run

```bash
cargo leptos build
```

### Running your project

Create an empty sqlite3 db file:

```bash
sqlite3 ./default.sqlite3 "VACUUM;"
```

Compile, run & watch for changes:

```bash
cargo leptos watch
```

## Deployment

### Compiling for Release

```bash
cargo leptos build --release
```

### Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/x86_64-unknown-linux-gnu/release`
2. The `site` directory and all files within located in `target/site`
3. An empty sqlite db file (`touch`ing the filename should be enough)

Copy these files to your remote server. The directory structure should be:

```text
server
default.sqlite3
site/
```

Set the following enviornment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="todo-vault"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_SITE_ROOT="./site"
```

Finally, run the server binary.

```text
./server
```

## Licensing

    TodoVault - Take back your data!
    Copyright (C) <2022>  <Tristan Druyen tristan[at]vault81.de>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
