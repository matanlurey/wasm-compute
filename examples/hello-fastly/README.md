# Hello Fastly

This is a simple example of a Fastly service written in Rust using the
[Fastly crate](https://crates.io/crates/fastly).

## Usage

```shell
fastly compute serve
```

## Publish

```shell
fastly compute publish
```

For this command to work, you will need to have a Fastly account and have
an API token, either via:

- pass `--token <token>` as an argument to the `publish` command;
- set a `FASTLY_API_TOKEN` environment variable;
- run `fastly profile <create>` (and verify with `fastly whoami`).

See <https://manage.fastly.com/account/personal/tokens>.

> [!NOTE]
> The token needs the API scope `global` to publish services.
