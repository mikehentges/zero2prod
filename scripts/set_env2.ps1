$Env:DATABASE_URL = "postgres://postgres:docker@localhost:5432/newsletter"
cd G:\Projects\rust\zero2prod
sqlx migrate run