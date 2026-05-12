# TMDB CLI Tool (Rust)

A simple command-line application built with Rust that fetches movie data from The Movie Database (TMDB) API and displays it directly in the terminal.

This project helps practice:

- API integration
- HTTP requests in Rust
- JSON parsing
- Environment variables
- Building CLI applications

---

## Features

- Fetch Now Playing movies
- Fetch Popular movies
- Fetch Top Rated movies
- Fetch Upcoming movies
- Clean terminal output
- Beginner-friendly project structure

---

## Technologies Used

- Rust
- TMDB API
- reqwest
- tokio
- serde
- clap
- dotenv
- env

---

## Installation

### Clone the Repository

```bash
git clone <your-repository-url>
cd tmdb-cli
```

### Install Rust

Make sure Rust is installed:

```bash
rustc --version
cargo --version
```

Download Rust from:

https://www.rust-lang.org/

---

## TMDB API Setup

1. Create an account on TMDB:

https://www.themoviedb.org/

2. Generate your API key

3. Create a `.env` file in the project root:

```env
TMDB_API_KEY=your_tmdb_api_key_here
```

---

## Usage

### Now Playing Movies

```bash
cargo run -- playing
```

### Popular Movies

```bash
cargo run -- popular
```

### Top Rated Movies

```bash
cargo run -- top
```

### Upcoming Movies

```bash
cargo run -- upcoming
```

---

## Example Output

```bash
==============================
 Now Playing Movies
==============================

1. Dune: Part Two
⭐ Rating: 8.5

2. Milagre 
⭐ Rating: 8.6
```

---

## Project Structure

```bash
tmdb-cli/
│
├── src/
│   └── main.rs
│   └── playing.rs
│   └── popular.rs
│   └── top.rs
│   └── upcoming.rs│
├── .env
├── Cargo.toml
└── README.md
```

---

## Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
clap = { version = "4.6.1", features = ["derive"] }
dotenv = "0.15.0"
env = "1.0.1"
```

---

## Learning Goals

This project helps practice:

- Async programming in Rust
- Working with REST APIs
- Handling JSON responses
- Error handling
- CLI argument parsing
- Environment configuration

---

## Future Improvements

- Search movies by title
- TV show support
- Pagination support
- Better terminal styling
- Save favorite movies
- Interactive CLI menu

---

## API Reference

TMDB API Documentation:

https://developer.themoviedb.org/docs/getting-started

---

## License

This project is licensed under the MIT License.
