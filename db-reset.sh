source ./.env
sqlx database reset
export DATABASE_URL=$DATABASE_URL_TEST
sqlx database reset