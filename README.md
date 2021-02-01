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

## TODO

Add the following routes:

- GET `/new_submissions`: lists the untreated submissions
    ```
    [
        {
            "id": "1", 
            "photos": ["url1", "url2", ...],
            "submission_date": "YYYY-MM-DDThh:mm:ss.sssZ"
        },
        {
            "id": "2", 
            "photos": ["url3", "url4", ...],
            "submission_date": "YYYY-MM-DDThh:mm:ss.sssZ"
        },
        {}
    ]
    ```
- POST `/change_submission_status`: changes the status of a submission