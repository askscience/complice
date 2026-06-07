# Complice

Anonymous "Mission of the Day" app. No login, no registration -- each client gets a session ID. Uses local LLM (Ollama) to generate location-aware missions based on your interests, weather, time of day, and recent activity.

## Stack

- **Backend:** Rust + Axum + SQLx (SQLite) + Reqwest
- **Frontend:** Vanilla HTML/CSS/JS (Inter font, glassmorphism, dark/light theme)
- **AI:** Ollama (`gemma4:31b-cloud` or any model)
- **Weather:** Open-Meteo API
- **Database:** SQLite (zero-config, single file)

## Quick Start

```bash
# 1. Start Ollama
ollama serve

# 2. Pull a model (if not already)
ollama pull gemma4:31b-cloud

# 3. Build and run
cp .env.example .env
cargo build --release
./target/release/complice
```

Open `http://localhost:3000`.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/health` | Database + Ollama status |
| GET | `/api/models` | List available Ollama models (cached 5 min) |
| POST | `/api/generate` | Generate 3 missions from AI (or fallback) |
| POST | `/api/complete` | Mark a mission as done, earn points |
| GET | `/api/stats?session_id=...` | Points, streak, badges, history |

## Configuration

Copy `.env.example` to `.env`:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Server port |
| `DATABASE_URL` | `sqlite:complice.db` | SQLite file path |
| `OLLAMA_URL` | `http://localhost:11434` | Ollama API URL |
| `DEFAULT_MODEL` | `gemma4:31b-cloud` | Default LLM model |
| `WEATHER_API_ENDPOINT` | `https://api.open-meteo.com/v1/forecast` | Weather API |

## Fallback Missions

If Ollama is unreachable or returns invalid JSON, the app falls back to 20 hand-curated rule-based missions covering photography, fitness, mindfulness, exploration, cooking, and more.

## Docker

```bash
docker-compose up -d
```

Starts Postgres, Ollama, and the backend. (Docker Compose uses the PostgreSQL variant; for local dev the default is SQLite.)

## Schema

- `sessions` -- session UUID, created/last-active timestamps
- `missions` -- generated missions with time limits, radius, points, 24h expiry
- `completed_missions` -- completion log with points earned
- `streaks` -- daily streak tracking per session
