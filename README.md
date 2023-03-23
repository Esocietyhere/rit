# RIT - Roblox Command-line Tool &emsp; ![Logo](icon.png)

[![CI](https://github.com/Esocietyhere/rit/workflows/CI/badge.svg)](https://github.com/Esocietyhere/rit/actions/workflows/ci.yml)
[![Release](https://github.com/Esocietyhere/rit/actions/workflows/release.yml/badge.svg?event=push)](https://github.com/Esocietyhere/rit/actions/workflows/release.yml)

Rit is a command-line tool that helps you manage your Roblox projects. It simplifies the process of uploading your project to Roblox, and makes it easy to build and open your Rojo projects. It can be configured to deploy between multiple experiences and places in the same project.

Inspired by the Roblox Open Cloud API using [Rbxcloud](https://github.com/Sleitnick/rbxcloud).

## Install CLI

### Aftman

Run the `aftman add` command within your project directory. This will add `rit` to the project's `aftman.toml` file (or create one if it doesn't yet exist).

```sh
$ aftman add Esocietyhere/rit@0.3.0
```

## Example Case

If you have a game that has both a staging and production version and you frequently need to deploy updates to both versions, it can be a hassle to manually configure the settings each time. However, you can streamline the process by using `rit` with a `config.json` file at the root of your project. This file can contain multiple experience IDs and place IDs, allowing you to easily switch between different environments without having to modify the configuration every time you deploy.

```json
{
  "deployment": {
    "universes": {
      "main": 4458588307
    },
    "places": {
      "main": {
        "default": 12721091425
      }
    }
  }
}
```

Then, you can use the `rit deploy -b [BRANCH_NAME]` command to deploy to the specified branch. If you don't specify a branch, it will default to `main`.
