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

## Starting a challenge

`just start-day <day-number>`

This will:
- Create a crate for the challenge
- get input, place it in files/input.txt in the crate; will fail if you do not have `SESSION_COOKIE` in `.env` in project root
- Overwrite main.rs with tools/base.rs, and add a justfile to the crate root
- Open the challenge via xdg-open

## Other notes

The year for challenge links is hardcoded.
