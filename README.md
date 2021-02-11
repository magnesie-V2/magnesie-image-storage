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

## API requests

### GET `/new_submissions`

Lists the untreated submissions

#### Parameters

None

#### Response

<table>
<tr>
    <th>Code</th>
    <th>Description</th>
</tr>
<tr>
    <td>200</td>
    <td>
        <pre lang="json">
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
            ...
        ]
        </pre>
    </td>
</tr>
</table>

### POST `/change_submission_status`

Changes the status of a given submission

#### Body - JSON

```json
{
    "id": 1,
    "status": "TREATED"
}
```

#### Response

<table>
<tr>
    <th>Code</th>
    <th>Description</th>
</tr>
<tr>
    <td>200</td>
    <td>OK: Updated</td>
</tr>
<tr>
    <td>400</td>
    <td>Bad Request: unknown id</td>
</tr>
<tr>
    <td>422</td>
    <td>Unprocessable entity: could not parse the JSON input</td>
</tr>
<tr>
    <td>500</td>
    <td>Internal Server Error: an error occured during the update</td>
</tr>
</table>

