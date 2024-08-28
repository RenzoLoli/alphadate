# Delete Alphabet

| URL                   | Method | Auth required | Description     |
| --------------------- | ------ | ------------- | --------------- |
| /api/v1/alphabets/:id | DELETE | YES           | Delete alphabet |

## URL Params

| Param | Type   | Description                      |
| ----- | ------ | -------------------------------- |
| id    | String | The id of the alphabet to delete |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "title": "string",
  "user_dates": [
    {
      "id": "string",
      "letter": "string",
      "completed": "string",
      "date_idea_id": "string"
    }
  ]
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

## Not Modified

**Code** : `304 NOT MODIFIED`

**Content** :

```json
{
  "message": "string"
}
```
