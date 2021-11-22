# Magnes.ie - Image Storage

## Run docker containers

```sh
docker-compose up
```

## Services

### Database

Creates a MySQL database schema at build. SQL scripts are executed in alphabetical order.

### WebService

Creates a Rust API service that automatically connects to the database. Hosts the following routes:

- `/`: homepage
- `/new_submissions`: lists the untreated submissions
- `/change_submission_status`: changes the status of a given submission
- `/submit`: handles the form data (submission, photo)
- `/hostedFiles`: opens access to the files in the `/hostedFiles` directory

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
        <pre>
[
    {
        "id": 1, 
        "photos": ["url1", "url2", …],
        "submission_date": "YYYY-MM-DDThh:mm:ss.sss"
    },
    {
        "id": 2, 
        "photos": ["url3", "url4", …],
        "submission_date": "YYYY-MM-DDThh:mm:ss.sss"
    },
    …
]</pre>
    </td>
</tr>
</table>

### POST `/change_submission_status`

Changes the status of a given submission

#### Body - JSON

```
{
    "id": 1,
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

### POST `/submit`

Handles the form data (submission, photo)

#### Body - multipart/form-data

```
photos: [File('photo1.jpg'), File('photo2.jpg'), File('photo3.jpg'), ...]
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
    <td>Bad Request: invalid form data</td>
</tr>
<tr>
    <td>422</td>
    <td>Unprocessable entity: could not parse the input</td>
</tr>
<tr>
    <td>500</td>
    <td>Internal Server Error: an error occured during the insertion</td>
</tr>
</table>
