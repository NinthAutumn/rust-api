To start development using autoreload:
cargo install systemfd cargo-watch

systemfd --no-pid -s http::{port} -- cargo watch -x run
