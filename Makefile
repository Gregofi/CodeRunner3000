
.PHONY: build prepare up all test

up: 
	docker compose up

prepare:
	$(MAKE) all ${languages} -C evaluator/infra

build:
	docker compose build

test: build
	docker compose -f compose.yaml -f integration_tests/evaluator.yaml run test

down:
	docker compose down
