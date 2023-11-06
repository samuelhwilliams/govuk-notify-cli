# Changelog

## v0.4.3

- Rename `notify db`'s `--aws` flag to `--paas`.
- By default, connect to AWS DB instance instead of GOV.UK PaaS.
- Rename the `--allow-writes` flag to just `--write`.
- Default to readonly role for AWS DBs.
- Support for elevating to write/admin roles on AWS DB access via `--write` and `--admin` flags.

## v0.4.2

- Bump rustix dependency.