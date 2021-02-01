# Magnes.ie - Image Storage

## Run docker containers
```sh
docker-compose up
```

## Services

### Database

Creates a MySQL database schema at build. SQL scripts are executed in alphabetical order.

### WebService

Creates a RUST API service that automatically connects to the database. Hosts the following routes:

- `/`: homepage
- `/users`: list the users
- `/files`: opens access to the files in the `/hostedFiles` directory