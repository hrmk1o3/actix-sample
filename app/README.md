# Actix Postgres Sample

## How to Use

### Procedure for Ubuntu

```sh
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql.service
sudo pg_ctlcluster 12 main start
createuser test_user
createdb -O test_user testing_db
```

```sh
cd app
psql -f sql/schema.sql testing_db
cp template.env .env
cargo run
```

Register a new user:

```sh
curl -X POST -d '{"email": "test@example.com", "first_name": "test", "last_name": "example", "username": "user1"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
```

Fetch all registered users:

```sh
curl http://127.0.0.1:8080/users
```

### Procedure for MacOS

```sh
brew install postgres
createuser -P test_user
createdb -O test_user testing_db
```

```sh
cd app
psql -f sql/schema.sql testing_db
cp template.env .env
cargo run
```

Register a new user:

```sh
curl -X POST -d '{"email": "test@example.com", "first_name": "test", "last_name": "example", "username": "user1"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
```

Fetch all registered users:

```sh
curl http://127.0.0.1:8080/users
```
