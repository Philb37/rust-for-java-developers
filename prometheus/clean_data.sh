#!/usr/bin/env bash

set -uo pipefail

# curl -X POST -g 'http://localhost:9090/api/v1/admin/tsdb/delete_series?match[]={__name__=~".+"}'
# curl -X POST 'http://localhost:9090/api/v1/admin/tsdb/clean_tombstones'

docker compose stop prometheus
docker compose rm -f prometheus
docker volume rm rust-for-java-developers_prometheus-data
docker compose up -d prometheus