# Fonctionnement Du Code

## Vue D'ensemble

Pixie est un serveur HTTP statique en Rust base sur:

- resolution de configuration YAML
- ecoute TCP
- routing de pages statiques HTML
- thread pool pour traiter les connexions

## Point D'entree

Fichier: `/pixie/src/main.rs`

Flux:

1. lit les arguments CLI (`pixie serve`)
2. charge la config runtime via `runtime_config()`
3. lance le serveur via `run_server(addr, workers)`

## Modules Principaux

### Configuration

Fichier: `/pixie/src/config.rs`

Responsabilites:

- lecture YAML depuis `PIXIE_CONFIG`, puis `./config-pixie.yml`, puis `/etc/pixie/config-pixie.yml`
- fallback sur valeurs hardcodees (`127.0.0.1:80`, `workers=4`)
- prise en charge des cles `addr`, `host`, `port`, `workers`, alias `address` et `nb_worker`

### Serveur

Fichier: `/pixie/src/server.rs`

Responsabilites:

- bind TCP sur l'adresse de config
- creation du pool de workers
- accept des connexions entrantes
- lecture de la premiere ligne HTTP et construction de la reponse

### Router

Fichier: `/pixie/src/router.rs`

Responsabilites:

- resolution du repertoire web (`PIXIE_WEB_ROOT`, fallback local/dev/system)
- mapping route -> fichier HTML (`/` -> `hello.html`, sinon `<route>.html`)
- retour 404 avec `404.html` si la page n'existe pas

### Thread Pool

Fichiers:

- `/pixie/src/threadpool/pool.rs`
- `/pixie/src/threadpool/worker.rs`
- `/pixie/src/threadpool/job.rs`

Responsabilites:

- file de jobs via canal mpsc
- workers qui executent les connexions en parallele
- fermeture propre des threads au drop

### Logs

Fichier: `/pixie/src/logger.rs`

Responsabilites:

- sorties `info`, `warn`, `error` standardisees

## Flux D'une Requete

1. un client ouvre une connexion TCP
2. `run_server` soumet la connexion au thread pool
3. `handle_connection` lit la request line
4. `resolve_route` choisit le fichier HTML
5. le serveur renvoie une reponse HTTP avec status + contenu

## Tests

Tous les tests sont regroupes dans:

- `/pixie/tests/`

Suites presentes:

- `config_tests.rs`
- `router_tests.rs`
- `server_tests.rs`
- `threadpool_tests.rs`
- `logger_tests.rs`
- `cli_tests.rs`

## Couverture

Commande recommandee (instrumentation LLVM):

```bash
cd pixie
cargo clean
RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='pixie-%p-%m.profraw' cargo test
TOOLBIN="$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin"
"$TOOLBIN/llvm-profdata" merge -sparse pixie-*.profraw -o pixie.profdata
"$TOOLBIN/llvm-cov" report \
  --object target/debug/deps/config_tests-* \
  --object target/debug/deps/threadpool_tests-* \
  --object target/debug/deps/router_tests-* \
  --object target/debug/deps/server_tests-* \
  --object target/debug/deps/logger_tests-* \
  --object target/debug/deps/cli_tests-* \
  --instr-profile pixie.profdata \
  --ignore-filename-regex '/\\.cargo/registry|/tests/'
```
