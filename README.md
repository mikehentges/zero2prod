# zero2prod
This is a windows-based build, so the following steps are needed to run the environment. All of this assumes sqlx is installed already.

1. run `docker compose up` to start the 3 services (postgres, collector, and jaeger)
2. run `scripts/set_env2.ps1` (powershell script) to set DATABASE_URL, and initial sqlx migrate
3. `cargo test subscribe_fails_if_there_is_a_fatal_database_error -- --nocapture` will reproduce the test I'm trying to get to work
