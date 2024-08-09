DOCKER_IMAGE = "ghcr.io/gregofi/compilers-cr3000:latest"

.PHONY: configure build all pull

all: build

configure:
	./bin/build-dockerfile.py ${LANGUAGES} > "Dockerfile"

build: configure
	docker build -t $(DOCKER_IMAGE) .

pull:
	docker pull $(DOCKER_IMAGE)

push: build
	docker push $(DOCKER_IMAGE)
