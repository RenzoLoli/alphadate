# Renew token

| URL                 | Method | Auth required | Description |
| ------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/renew/ | POST   | YES           | Renew token |

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "token": "string"
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
