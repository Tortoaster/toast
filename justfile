run:
    cd backend && cargo run
up:
    docker compose up -d
down:
    docker compose down -v
gen:
    cd backend && DATABASE_URL=postgres://user:password@localhost/toast cargo sqlx prepare && cd ../frontend && deno i --allow-scripts && deno task prepare
dev:
    cd frontend && deno task dev
fmt:
    cd backend && cargo +nightly fmt && cd ../frontend && deno fmt
