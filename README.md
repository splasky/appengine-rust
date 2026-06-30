# appengine-rust

Multi-platform Rust deployment template supporting **GCP App Engine (flex)**, **Docker**,
**Kubernetes**, **Cloudflare Workers**, and **Vercel**.

Built with [actix-web](https://actix.rs/) 4.x on Rust nightly (edition 2024).

## Quick Start

```bash
cargo build
cargo run
# Visit http://localhost:8080
# Health check at http://localhost:8080/health
```

## Endpoints

| Path | Method | Description |
|------|--------|-------------|
| `/` | GET | Service info (name, version) |
| `/health` | GET | Health check (200 OK) |

## Deploy

### GCP App Engine (flex)

```bash
cd gcp-appengine
gcloud auth login
gcloud app deploy
```

### Docker

```bash
docker build -t appengine-rust .
docker run -p 8080:8080 -e RUST_LOG=info appengine-rust
```

Or with Docker Compose:

```bash
docker compose -f Docker/docker-compose.yml up
```

### Kubernetes

```bash
kubectl apply -f k8s/
```

Services are exposed on port 80 via ClusterIP. Update `k8s/ingress.yaml` with your domain.

### Cloudflare Workers (Rust → WASM)

```bash
cd cloudflare
wasm-pack build --target web --release
npx wrangler deploy
```

Or using npm scripts:

```bash
cd cloudflare
npm run deploy
```

### Vercel (Rust native)

```bash
cd Vercel
vercel deploy
```

The Vercel config uses the `@vercel/rust` builder to compile `src/main.rs` (actix-web server) to a native binary. Routes are handled directly by the server — no reverse proxy needed.

## CI/CD

GitHub Actions workflows:

| Workflow | Trigger | What it does |
|----------|---------|-------------|
| **CI** | Push / PR | Build, test, clippy lint, format check, coverage report |
| **Release** | `v*.*.*` tag | Multi-arch build (linux amd64/arm64, macOS amd64/arm64, Windows amd64), GitHub Release with archives |

### Release a new version

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Project Structure

```
├── src/main.rs              # Actix-web HTTP server
├── Cargo.toml               # Rust edition 2024, nightly toolchain
├── Dockerfile               # Multi-stage Docker build
├── Docker/                  # Docker Compose
├── gcp-appengine/           # GCP App Engine (flex) config
├── k8s/                     # Kubernetes manifests
├── cloudflare/              # Cloudflare Workers — Rust WASM crate
├── Vercel/                  # Vercel — @vercel/rust native binary
└── .github/workflows/       # CI/CD pipelines
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | Listen port |
| `RUST_LOG` | (unset) | Log level (e.g., `info`, `debug`) |

## License
Apache 2.0
