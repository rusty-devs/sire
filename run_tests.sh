#!/usr/bin/env bash
# -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
# Helper script iused to run the unit test suite in "instrumented" mode
# generating code coverage metrics
# -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

# Fail on first error
set -e
# Show output from commands run in this script
set -x

# To run before running this script:
# rustup default nightly
# rustup component add llvm-tools-preview
rustup show

# Configure the Rust environment to allow for instrumentation
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
#export RUSTFLAGS="-C instrument-coverage"
#export LLVM_PROFILE_FILE="sire-%p-%m.profraw"

# Rebuild test suite using instrumented code
#cargo clean
cargo test

rm -if lcov.info
rm -if ccov.zip
rm -rf coverage

# Ignore coverage report if grcov is not installed
hash grcov 2>/dev/null || exit 0;

# Generate coverage report from instrumented output
# This technique generates coverage data just for the files associated
# with the current project
# Requires the lcov package installed to get the genhtml tool (brew install lcov)
#zip -0 ccov.zip "$(find . \( -name "sire*.gc*" \) -print)";
#grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o lcov.info
#genhtml -o coverage/ --show-details --highlight --ignore-errors source --legend lcov.info

# Alternative logic for generating coverage reports which yields slightly ifferent results
grcov . --binary-path target/debug -s ./src -t lcov --branch --ignore "/*" -o ./lcov.info
grcov . --binary-path target/debug -s ./src -t html --branch --ignore "/*" -o ./coverage

open ./coverage/index.html