# RIT - Roblox Command-line Tool &emsp; ![Logo](icon.png)

[![CI](https://github.com/Esocietyhere/rit/workflows/CI/badge.svg)](https://github.com/Esocietyhere/rit/actions/workflows/ci.yaml)

Rit is a command-line tool that helps you manage your Roblox projects. It simplifies the process of uploading your project to Roblox, and makes it easy to build and open your Rojo projects. It can be configured to deploy between multiple experiences and places in the same project.

Inspired by the Roblox Open Cloud API using [Rbxcloud](https://github.com/Sleitnick/rbxcloud).

&nbsp;

## Install CLI

### Aftman

Run the `aftman add` command within your project directory. This will add `rit` to the project's `aftman.toml` file (or create one if it doesn't yet exist).

```sh
$ aftman add Esocietyhere/rit@0.1.0
```

## Example Case

You have two versions of the same game: A staging version and a production version. You want to be able to deploy to both of them at the same time, but you don't want to have to change the configuration every time you want to deploy between the staging and production environment. You can use `rit` with a config.json at the root of your project which includes multiple experience IDs and place IDs.

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

Then, you can use the `rit deploy -b [BRANCH_NAME]` command to deploy to either of the environments.

```sh

```
