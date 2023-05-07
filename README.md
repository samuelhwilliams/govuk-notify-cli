# GOV.UK Notify CLI

A thin wrapper CLI around some common commands that GOV.UK Notify developers might want to run.

## Functionality

* SSH onto an app
* Connect to a database
* Log into AWS Console / run a command in a Notify AWS account

## Dependencies

This CLI depends on quite lot of other utilities being installed and already configured as per our [getting started](https://github.com/alphagov/notifications-manuals/wiki/Getting-started) docs:

* [gds-cli](https://github.com/alphagov/gds-cli)
* [cf-cli](https://github.com/cloudfoundry/cli/blob/main/doc/installation-instructions/installation-instructions-v7.md#installers-and-compressed-binaries)
* [aws session manager](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html#install-plugin-macos)

