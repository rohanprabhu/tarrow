set -e

docker compose -f test-setup/docker-compose.yaml down
docker compose -f test-setup/docker-compose.yaml up --wait

.local-root/bin/diesel migration run --database-url 'postgres://tarrow-dev:tarrow-dev@localhost:35431/tarrow-test-db'

if [ "$1" == "watch" ]; then
  cargo watch -x "test -- --nocapture"
else
  cargo test -- --nocapture
fi

