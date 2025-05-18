run:
    cd backend && cargo run
up:
    docker compose up -d
down:
    docker compose down -v
gen:
    cd backend && DATABASE_URL=sqlite://../sqlite.db cargo sqlx prepare && cd ../frontend && deno i --allow-scripts && deno task gen
dev:
    cd frontend && deno task dev
fmt:
    cd backend && cargo +nightly fmt && cd ../frontend && deno fmt
