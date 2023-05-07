# GOV.UK Notify CLI

A thin wrapper CLI around some common commands that GOV.UK Notify developers might want to run.

## Functionality

* SSH onto an app
* Connect to a database
* Log into AWS Console / run a command in a Notify AWS account

## Shell completions

Shell completions are available from `notify --generate <shell>`. If you want these always available, you can add something like this to your `~/.bash_profile`:

```bash
source <(notify --generate bash 2>/dev/null)
```

## Dependencies

This CLI depends on quite lot of other utilities being installed and already configured as per our [getting started](https://github.com/alphagov/notifications-manuals/wiki/Getting-started) docs:

* [gds-cli](https://github.com/alphagov/gds-cli)
* [cf-cli](https://github.com/cloudfoundry/cli/blob/main/doc/installation-instructions/installation-instructions-v7.md#installers-and-compressed-binaries)
* [aws session manager](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html#install-plugin-macos)
* [notifications-aws](https://github.com/alphagov/notifications-aws/) checked out.
    * Add `export NOTIFY_AWS=<path_to_git_repo>` to your `~/.bash_profile` or equivalent. This environment variable will be read as a default value when trying to run certain scripts from the notifications-aws repo.
