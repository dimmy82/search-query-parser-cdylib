#! /bin/bash

./fmt_check_test.sh
cargo clean && cargo build --release
