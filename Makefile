run:
	cargo run


db:
	docker-compose up -d

db-clean:
	docker-compose down --volumes
