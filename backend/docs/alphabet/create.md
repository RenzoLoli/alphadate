# Create Alphabet

| URL                        | Method | Auth required | Description     |
| -------------------------- | ------ | ------------- | --------------- |
| /api/v1/alphabets/:user-id | POST   | YES           | Create alphabet |

## URL Params

| Param   | Type   | Description                               |
| ------- | ------ | ----------------------------------------- |
| user_id | String | The id of the user to create alphabet for |

## Request Body

```json
{
  "title": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "title": "string"
}
```

## Bad Request Response

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "string"
}
```

## Unauthorized Response

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "message": "string"
}
```

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
