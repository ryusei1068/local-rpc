#!/bin/bash

if [ $# != 1 ]; then
	echo "Wrong number of arg"
	echo "./docker-operation.sh run or remove"
	exit 1
fi

ARG=$1

if [ $ARG = "run" ]; then
	docker build -t rpc .
	docker run --rm --name rpc rpc:latest
elif [ $ARG = "remove" ]; then
	docker rmi rpc:latest
fi

