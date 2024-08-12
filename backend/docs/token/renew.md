# Renew token

Used to renew requested tokens.

**URL** : `/api/v1/token/renew`

**Method** : `POST`

**Auth required** : YES

## Success Response

**Code** : `200 OK`

**Content**

```json
{
  "token": "[token generated]"
}
```

## Bad Request Response

**Condition** : bad token request.

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "[Cause of the error]"
}
```

## Not Modified

**Condition** : Some server error or token is invalid.

**Code** : `304 NOT MODIFIED`

**Content** :

```json
{
  "message": "Cannot renew token"
}
```
