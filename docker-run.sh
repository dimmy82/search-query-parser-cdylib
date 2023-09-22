#!/bin/zsh

rm -fr docker/target
mkdir -p docker/target

docker build --no-cache -t build-search-query-parser-cdylib -f docker/Dockerfile .

docker run --rm -v $(realpath docker/target):/search-query-parser-cdylib/target build-search-query-parser-cdylib
