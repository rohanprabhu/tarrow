set -e

docker compose -f test-setup/docker-compose.yaml down
docker compose -f test-setup/docker-compose.yaml up -d
if [ "$1" == "watch" ]; then
  ec=`cargo watch -x "test -- --nocapture"`
else
  ec=`cargo test -- --nocapture`
fi

docker compose -f test-setup/docker-compose.yaml down

