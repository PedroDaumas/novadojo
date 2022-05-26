all: build 

COMPOSE_FILE=./docker/docker-compose.yml

DOCKER_COMPOSE=docker-compose -f ${COMPOSE_FILE}
MAIN_SERVICE=main

DOCKER_COMPOSE_RUN=${DOCKER_COMPOSE} run --rm
DOCKER_COMPOSE_UP=${DOCKER_COMPOSE} up --remove-orphans
DOCKER_COMPOSE_BUILD=${DOCKER_COMPOSE} build 

run:
	${DOCKER_COMPOSE_RUN} ${MAIN_SERVICE}