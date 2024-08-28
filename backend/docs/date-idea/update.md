# Update Date Idea

| URL                   | Method | Auth required | Description      |
| --------------------- | ------ | ------------- | ---------------- |
| /api/v1/date-idea/:id | PUT    | YES           | Update date idea |

## URL Params

| Param | Type   | Description                       |
| ----- | ------ | --------------------------------- |
| id    | String | The id of the date idea to update |

## Request Body

```json
{
  "idea": "string",
  "description": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "idea": "string",
  "description": "string"
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
