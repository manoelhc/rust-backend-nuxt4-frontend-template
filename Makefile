run:
	cp .env.example .env
	docker-compose up --build -d

up: run

stop:
	docker-compose down

down: stop

logs:
	docker-compose logs -f

build:
	docker-compose build --no-cache