Créez un container docker avec la commande suivante

```sh
docker run --name rustodo-postgres-db \
    -e POSTGRES_PASSWORD=postgres_password \
    -e POSTGRES_USER=postgres_user \
    -e POSTGRES_DB=rustodo-rest-api \
    -p 5432:5432 \
    -d postgres
```