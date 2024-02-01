DB_DOCKER_CONTAINER=postgres-dev

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"

install-cli:
	cargo install sqlx-cli

fmt:
	cargo fmt

check:
	cargo check

build:
	cargo build

run:
	cargo run

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root actixwebapi

start_docker_db:
	docker start ${DB_DOCKER_CONTAINER}