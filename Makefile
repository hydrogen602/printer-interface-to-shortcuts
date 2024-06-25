SHELL := /bin/bash

HOST := $(shell docker context list | grep 'homepi' | awk '{ print $$3 }')

TAG_NAME := hydrogen602/printer-actions

make:
	docker image build -t "${TAG_NAME}" .

make-remote:
	DOCKER_HOST=${HOST} docker image build -t "${TAG_NAME}" .

# build and deploy the rust container
deploy: stop-deploy make-remote
	docker-compose --context homepi up -d

# helper
stop-deploy:
	docker-compose --context homepi down

# check the status of the containers
ps:
	DOCKER_HOST=${HOST} docker-compose ps -a

# check the logs of the rust container
logs:
	DOCKER_HOST=${HOST} docker logs --follow printer-actions-printer-actions-1
