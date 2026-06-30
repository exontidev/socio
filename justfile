set windows-shell := ["cmd.exe", "/c"]

up:
    docker compose up -d


down:
    docker compose down

logs:
    docker compose logs -f

monitoring-up:
    docker compose -f monitoring/docker-compose.yaml up -d

monitoring-down:
    docker compose -f monitoring/docker-compose.yaml down
