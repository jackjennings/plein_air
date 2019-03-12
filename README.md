# Plein Air

An overly-opinionated web server for publishing plain text documents.

## Development

```shell
ROCKET_CONTENT_DIRECTORY=path/to/directory cargo run
```

## Production

```shell
cargo build --production

ROCKET_CONTENT_DIRECTORY=path/to/directory \
  ROCKET_SECRET_KEY="AFlefuNeEVZIlVOd6ouvfAm1SVQ1lOTgo8rHGh9n8Es=" \
  ./target/release/plein_air
```
