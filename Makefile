.PHONY: fmt check test migrate up
fmt:
	cargo fmt --all
check:
	cargo check --workspace --all-targets
test:
	cargo test --workspace
migrate:
	cargo run -p fractio-backend -- migrate
up:
	docker compose up --build
