# Todo Vault API

## Overview

This is a webapp to manage and share todo lists.
It tries to be simple and fast.
In the longer term we also want to offer improved privacy & security by E2E encryption of notes/note-lists.

### Stack

The current stack includes (but is not limited too):

- DB: [sqlite3](https://sqlite.org/index.html)
- ORM: [SeaORM](https://www.sea-ql.org/SeaORM/)
- Server Framework: [Axum](https://github.com/tokio-rs/axum)
- Web App Framework(with hydration+SSR): [leptos](https://github.com/leptos-rs/leptos)
- CSS Framework [TailwindCSS](Web App Framework(with hydration+SSR): [leptos](https://github.com/leptos-rs/leptos))

### Server System Requirements

- CPU which supports the x86-64-v3 instruction set optimizations. (Intel Haswell and newer, AMD Excavator and newer)
- A recent linux kernel

These only apply to default build settings and to the provided pre-built binaries and container images.
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

```bash
cargo leptos watch
```

### Compiling for Release

```bash
cargo leptos build --release
```

### Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
todo-vault
site/
```

Set the following enviornment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="todo-vault"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
```

Finally, run the server binary.

## Licensing

    MyVault - Take back your data!
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


    If the AGPL license does not suit your needs, please contact me
    via the above email address to purchase an MIT licensed version.

# auth_vault_api

[![Build & Test](https://github.com/vault81/auth_vault_api/actions/workflows/build.yml/badge.svg)](https://github.com/vault81/auth_vault_api/actions/workflows/build.yml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/vault81/auth_vault_api)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/vault81/auth_vault_api)](https://rust-reportcard.xuri.me/report/github.com/vault81/auth_vault_api)
[![license-badge](https://img.shields.io/github/license/vault81/auth_vault_api)](https://github.com/vault81/auth_vault_api/blob/main/LICENSE.md)

⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️

## Vision

1. Rent a server
2. Install this
3. Select whatever service you want
4. ???
5. Profit

## Staged Goals

- Stage 1
  - Fleshed out user management incl. SSO, Oauth, OpenID, SAML & LDAP (at least 3)
- Stage 2
  - Gateway capabilities to provide secure access to unsecured http/s services by proxying them
- Stage 3
  - Docker integration to auto-setup services with the gateway
- Stage 4
  - Docker integration for services with their own outh via templates + setup/scripts (e.g. to enable Nextclouds Oauth Plugin and set it up)
- Stage 5
  - Simple configurability for the services
- Stage 6
  - Advanceded configurability for the services
- Stage 6-9000
  - Add Templates for X

## Dev Setup

- 1. You'll rustup for installing the needed nightly toolchains
- 2. Install cargo make
  ```sh
     cargo install --locked cargo-make
  ```
- 3. install other cargo+npm deps
  ```sh
     cargo make setup_deps
  ```
- 3. Run the dev watch service
  ```sh
     cargo make watch
  ```
- 4. Visit `http://localhost:8080`
- 5. ????
- 6. Profit !

## Licensing

    MyVault - Take back your data!
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


    If the AGPL license does not suit your needs, please contact me
    via the above email address to purchase an MIT licensed version.# auth_vault_api

[![Build & Test](https://github.com/vault81/auth_vault_api/actions/workflows/build.yml/badge.svg)](https://github.com/vault81/auth_vault_api/actions/workflows/build.yml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/vault81/auth_vault_api)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/vault81/auth_vault_api)](https://rust-reportcard.xuri.me/report/github.com/vault81/auth_vault_api)
[![license-badge](https://img.shields.io/github/license/vault81/auth_vault_api)](https://github.com/vault81/auth_vault_api/blob/main/LICENSE.md)

⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️ ⚠️ WIP ⚠️

## Vision

1. Rent a server
2. Install this
3. Select whatever service you want
4. ???
5. Profit

## Staged Goals

- Stage 1
  - Fleshed out user management incl. SSO, Oauth, OpenID, SAML & LDAP (at least 3)
- Stage 2
  - Gateway capabilities to provide secure access to unsecured http/s services by proxying them
- Stage 3
  - Docker integration to auto-setup services with the gateway
- Stage 4
  - Docker integration for services with their own outh via templates + setup/scripts (e.g. to enable Nextclouds Oauth Plugin and set it up)
- Stage 5
  - Simple configurability for the services
- Stage 6
  - Advanceded configurability for the services
- Stage 6-9000
  - Add Templates for X

## Dev Setup

- 1. You'll rustup for installing the needed nightly toolchains
- 2. Install cargo make
  ```sh
     cargo install --locked cargo-make
  ```
- 3. install other cargo+npm deps
  ```sh
     cargo make setup_deps
  ```
- 3. Run the dev watch service
  ```sh
     cargo make watch
  ```
- 4. Visit `http://localhost:8080`
- 5. ????
- 6. Profit !

## Licensing

    TodoVault
    Copyright (C) <2023>  <Tristan Druyen tristan[at]vault81.de>

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
