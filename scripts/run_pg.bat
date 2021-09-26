REM docker run --rm   --name pg-docker -e POSTGRES_PASSWORD=docker -p 5432:5432 -v g:/Projects/postgres_docker/data:/var/lib/postgresql/data  postgres
docker run --rm   --name pg-docker ^
    -e POSTGRES_PASSWORD=docker ^
    -e POSTGRES_USER=postgres ^
    -e POSTGRES_DB=newsletter ^
    -d -p 5432:5432 ^
    postgres