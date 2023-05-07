# GOV.UK Notify CLI

A thin wrapper CLI around some common commands that GOV.UK Notify developers might want to run.

## Functionality

* SSH onto an app (PaaS / AWS)
* Connect to a database (PaaS / AWS)
* Log into AWS Console / run a command in a Notify AWS account

## Installation

The easiest (albeit slowest) way to install is via [homebrew](https://brew.sh/). This will also install some of the required binary dependencies listed below.

```
brew tap samuelhwilliams/govuk-notify
brew install govuk-notify-cli
```

The brew formula is at[samuelhwilliams/homebrew-govuk-notify](https://github.com/samuelhwilliams/homebrew-govuk-notify).

### Dependencies

This CLI depends on quite lot of other utilities being installed and already configured as per our [getting started](https://github.com/alphagov/notifications-manuals/wiki/Getting-started) docs:

#### Binaries

* [gds-cli](https://github.com/alphagov/gds-cli)
    * [aws-vault](https://github.com/99designs/aws-vault)
* [cf-cli](https://github.com/cloudfoundry/cli/blob/main/doc/installation-instructions/installation-instructions-v7.md#installers-and-compressed-binaries)
* [aws session manager](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html#install-plugin-macos)

#### Git repositories

* [notifications-aws](https://github.com/alphagov/notifications-aws/) checked out.
    * Add `export NOTIFY_AWS=<path_to_git_repo>` to your `~/.bash_profile` or equivalent. This environment variable will be read as a default value when trying to run certain scripts from the notifications-aws repo.

## Shell completions

Shell completions are available from `notify --generate <shell>`. If you want these always available, you can add something like this to your `~/.bash_profile`:

```bash
source <(notify --generate bash 2>/dev/null)
```
