# build project
build:
	cargo build

# run project
run:
	cargo run

# check project for code smells
check:
	cargo clippy -- -Dclippy::correctness -Wclippy::complexity -Wclippy::perf -Wclippy::pedantic

# run tests
test:
	cargo test

# run tests
nextest:
	cargo nextest run

# watch files and check for errors
watch-check:
	watchexec -c -r -e rs,toml,lock -- just check

# watch files and run tests
watch-test:
	watchexec -c -r -e rs,toml,lock -- just test

# watch files and run tests
watch-nextest:
	watchexec -c -r -e rs,toml,lock -- just nextest


# NIX COMMANDS

# build project with nix
nix-build:
	nix build .#
# run project with nix
nix-run:
	nix run .#
# check project with nix
nix-check:
	nix build .#check
# test project with nix
nix-test:
	nix build .#test"

