# RIT - Roblox Command-line Tool ![Logo](icon.png)

[![CI](https://github.com/Esocietyhere/rit/workflows/CI/badge.svg)](https://github.com/Esocietyhere/rit/actions/workflows/ci.yml)
[![Release](https://github.com/Esocietyhere/rit/actions/workflows/release.yml/badge.svg?event=push)](https://github.com/Esocietyhere/rit/actions/workflows/release.yml)

Rit is a command-line tool that helps you manage your Roblox projects. It simplifies the process of uploading your project to Roblox and makes it easy to build and open them. It can be configured to deploy between multiple experiences and places in the same project.

Inspired by the Roblox Open Cloud API using [Rbxcloud](https://github.com/Sleitnick/rbxcloud).

## Install CLI

### Aftman

Run the `aftman add` command within your project directory. This will add `rit` to the project's `aftman.toml` file (or create one if it doesn't yet exist).

```sh
$ aftman add Esocietyhere/rit@0.3.1
```

## Example Case

Managing a Roblox project with multiple places can be challenging, especially when it comes to publishing which usually takes several tools to function. You can simplify the process by utilizing `rit` along with a `config.json` file placed at the root of your project. This file can store multiple experience IDs and place IDs, making it easy to switch between different environments without having to modify the configuration every time you deploy.

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
