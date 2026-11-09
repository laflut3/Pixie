# Architecture and request flow

Pixie serves static HTML files over HTTP. The binary starts the server; the library contains configuration, routing, logging, and worker-pool code.

## Main components

| File | Responsibility |
| --- | --- |
| `src/main.rs` | Loads runtime configuration and starts the server. |
| `src/config.rs` | Reads YAML settings and applies defaults. |
| `src/server.rs` | Accepts TCP connections and writes HTTP responses. |
| `src/router.rs` | Maps `GET` paths to HTML files. |
| `src/threadpool/` | Runs connection handlers on worker threads. |
| `src/logger.rs` | Writes info, warning, and error messages to stderr. |

## Request flow

1. Pixie accepts a TCP connection.
2. A worker reads the first HTTP request line.
3. The router maps `GET /` to `hello.html` and `GET /about` to `about.html`.
4. A query string is ignored when selecting the page.
5. Pixie returns the page with `200 OK`, or uses `404.html` with `404 NOT FOUND` when the route is invalid or missing.

Responses use `Content-Type: text/html; charset=utf-8` and include a content length. The server currently reads only the request line and serves HTML files.

## Web root lookup

Pixie uses the first available directory in this order:

1. The directory named by `PIXIE_WEB_ROOT`, if it exists.
2. The repository's `web/` directory.
3. `/usr/share/pixie/web`.

The development directory is checked before the system directory, so a repository `web/` folder takes precedence when present.
