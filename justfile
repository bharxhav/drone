[default]
[private]
default:
    @just --list

# Check the workspace or a package.
[group('porcelain')]
check package="": (fmt-check package) (clippy package) (test package)

# Fix the workspace or a package.
[group('porcelain')]
fix package="": (fmt package) (clippy-fix package)

# Build the workspace or a package.
[group('porcelain')]
build package="":
    cargo build {{ if package == "" { "--workspace" } else { "--package " + quote(package) } }} --all-targets

# Run the drone CLI.
[group('porcelain')]
[positional-arguments]
run *args:
    cargo run --package drone -- "$@"

# Build a dated release of the drone CLI.
[group('porcelain')]
release date:
    DRONE_RELEASE_DATE={{ quote(date) }} cargo build --release --package drone

# Check formatting for the workspace or a package.
[group('read-only')]
fmt-check package="":
    cargo fmt {{ if package == "" { "--all" } else { "--package " + quote(package) } }} --check

# Lint the workspace or a package.
[group('read-only')]
clippy package="":
    cargo clippy {{ if package == "" { "--workspace" } else { "--package " + quote(package) } }} --all-targets --all-features -- -D warnings

# Test the workspace or a package.
[group('read-only')]
test package="":
    cargo test {{ if package == "" { "--workspace" } else { "--package " + quote(package) } }} --all-features

# Format the workspace or a package.
[group('write')]
fmt package="":
    cargo fmt {{ if package == "" { "--all" } else { "--package " + quote(package) } }}

# Apply Clippy fixes to the workspace or a package.
[group('write')]
clippy-fix package="":
    cargo clippy --fix {{ if package == "" { "--workspace" } else { "--package " + quote(package) } }} --all-targets --all-features --allow-dirty --allow-staged -- -D warnings
