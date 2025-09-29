
.PHONY: build prepare up all test

up: build 
	docker compose up

prepare:
	$(MAKE) all ${languages} -C evaluator/infra

build:
	docker compose build

test: build
	docker compose -f compose.yaml -f integration_tests/evaluator.yaml run --build test

down:
	docker compose down
