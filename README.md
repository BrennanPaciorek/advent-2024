# AOC 2024 - Rust
Root directory for advent of code

## Prerequisites
### Dependencies
- Linux system
- rust
- cargo
- git
- just (via cargo)
- xdg-desktop-portal (via package manager)

### dotenv variables

the root Justfile will read from `.env` for `SESSION_COOKIE`. `SESSION_COOKIE` should be an AoC session cookie:
```
SESSION_COOKIE=<aoc-session-cookie>
```

## Creating a crate for a challenge

`just get-day <day-number>`

The year is hardcoded.
