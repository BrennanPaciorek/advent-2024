set dotenv-filename := ".env"

# get input from day (int 1-25)
get-input day:
    curl -b "session=$SESSION_COOKIE" https://adventofcode.com/2024/day/{{ day }}/input

# Create crate for day
get-day day:
    #!/bin/bash
    export CRATE_NAME=day-{{day}}
    cargo new --vcs none $CRATE_NAME
    cp tools/base.rs $CRATE_NAME/src/main.rs
    cp tools/Justfile $CRATE_NAME/Justfile
    mkdir $CRATE_NAME/files
    touch $CRATE_NAME/files/sample.txt
    just get-input {{day}} > $CRATE_NAME/files/input.txt
    xdg-open https://adventofcode.com/2024/day/{{ day }}
