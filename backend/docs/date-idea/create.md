# Create Date Idea

| URL                | Method | Auth required | Description      |
| ------------------ | ------ | ------------- | ---------------- |
| /api/v1/date-idea/ | POST   | YES           | Create date idea |

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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
