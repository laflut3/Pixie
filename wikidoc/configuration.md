# Configuration

Pixie reads an optional YAML file. Set `PIXIE_CONFIG` to select a specific file:

```sh
PIXIE_CONFIG=infrastructure/config/config-pixie.yml cargo run --locked
```

## File lookup

When `PIXIE_CONFIG` is unset or empty, Pixie checks:

1. `./config-pixie.yml`
2. `/etc/pixie/config-pixie.yml`
3. Built-in defaults, if neither file exists

When `PIXIE_CONFIG` names a file that does not exist, Pixie uses built-in defaults. Invalid YAML or a file read error stops startup.

## Settings

| Setting | Meaning | Default |
| --- | --- | --- |
| `addr` | Listen address in `host:port` format. | `127.0.0.1:80` |
| `host` | Host used when `addr` is not set. | `127.0.0.1` |
| `port` | Port used when `addr` is not set. | `80` |
| `workers` | Number of worker threads. | `4` |

`addr` takes precedence over `host` and `port`. The aliases `address` and `nb_worker` are also accepted. A worker count of zero falls back to `4`.

The sample file at `infrastructure/config/config-pixie.yml` listens on `0.0.0.0:8080` with four workers.
