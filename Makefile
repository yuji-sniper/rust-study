down:
	docker compose down

shell:
	docker compose exec app bash

cargo-run:
	docker compose exec app cargo run
