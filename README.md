# Chess Pace Tracker

Chess Pace Tracker turns a Lichess player's recent rated games into a quick view of their time management. Instead of looking only at results, it asks a more useful practical question: was the player ahead or behind on the clock by the middle of the game?

![Chess Pace Tracker interface](https://github.com/user-attachments/assets/ee70fe19-dc85-4f54-9f33-85604e60b158)

## What it does

Enter a Lichess username, time control, colour, and sample size (up to 50 games). The application then:

- streams the selected rated games from the Lichess API;
- calculates each game's clock difference at the halfway point;
- summarizes the average time advantage or deficit;
- shows the trend across the sample, win rate, and flagging counts; and
- reports games that could not be included rather than silently discarding them.

The progress indicator is driven by a WebSocket connection, so it advances as individual games are processed rather than waiting for the whole request to finish.

## How it is built

```text
React + TypeScript UI
        |
        | POST /fetch-chess-data, WebSocket progress updates
        v
Rust / Actix Web service
        |
        | NDJSON game stream
        v
Lichess API
```

The backend keeps data fetching, deserialization, game analysis, insight generation, and request logging in separate modules. It uses SQLite for request-timing data; this was useful for comparing analysis requests while developing the app.

## Run locally

Prerequisites: a current Rust toolchain and Node.js (the frontend was built with Create React App).

Start the backend first:

```bash
cd backend
cargo run
```

In a second terminal, start the frontend:

```bash
cd frontend
npm install
npm start
```

Open `http://localhost:3000`. The frontend expects the API at `http://localhost:8000`.

## Scope and trade-offs

This is a focused analysis tool, not a chess engine or an attempt to evaluate move quality. The pace metric deliberately compares the remaining clock at the game's halfway point, which makes it easy to interpret across a sample but does not account for position complexity, increment usage, or time spent before a game reaches the midpoint.

The project was built around the public Lichess API and is intended for local use. A production version would add configuration for API and frontend origins, stronger automated coverage, and a deployable full-stack setup.

## Repository layout

```text
frontend/   React and TypeScript interface, charts, and WebSocket client
backend/    Actix Web API, Lichess stream processing, and analysis logic
```

## Technology

React, TypeScript, Tailwind CSS, ApexCharts, Rust, Actix Web, Tokio, WebSockets, SQLite, and the Lichess API.
