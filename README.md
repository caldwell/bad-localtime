# bad-localtime

see:
- <https://github.com/chronotope/chrono/issues/499>
- <https://github.com/time-rs/time/issues/293>

To test chrono and time libraries, edit `Cargo.toml` to point to the version you
want to test then:

    cargo run --bin time
    cargo run --bin chrono
