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
- `/users`: list the users
- `/sites`: list the sites
- `/new_submissions`: lists the untreated submissions
- `/change_submission_status`: changes the status of a given submission
- `/submit`: handles the form data (user, site, submission, photo)
- `/hostedFiles`: opens access to the files in the `/hostedFiles` directory

## API requests

### GET `/users`

Lists the users

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
        "name": "Jean"
    },
    {
        "id": 2, 
        "name": "Pierre"
    },
    …
]</pre>
    </td>
</tr>
</table>

### GET `/sites`

Lists the sites

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
        "name": "Site A"
        "details": "Site A is a wonderful climbing spot"
        "latitude": "50.0"
        "longitude": "2.0"
    },
    {
        "id": 2, 
        "name": "Site B"
        "details": "Site B is a great climbing spot"
        "latitude": "55.0"
        "longitude": "-2.0"
    },
    …
]</pre>
    </td>
</tr>
</table>

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

Handles the form data (user, site, submission, photo)

#### Body - multipart/form-data

```
user_id: "0"
user_name: "Lucas"
site_id: "0"
site_name: "Fontainebleau"
site_details: "Le site naturel d'escalade de Fontainebleau, communément désigné par Bleau par les grimpeurs (ou Font en anglais) est situé dans la forêt de Fontainebleau et aux alentours. Internationalement réputé pour le bloc, discipline d'escalade sur des rochers d'une faible hauteur ne nécessitant en général pas de corde pour l'assurance, Bleau est composé d'une centaine de massifs et de milliers de blocs de grès."
site_latitude: "48.4065"
site_longitude: "2.6201"
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

