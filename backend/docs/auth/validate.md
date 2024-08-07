# Validate

Used to validate Token.

- !only for test JWT integration

**URL** : `/api/v1/auth/validate/`

**Method** : `POST`

**Auth required** : NO

**Header Constraits**

```json
{
  "Authorization": "Bearer [valid token]"
}
```

**Header example**

```json
{
  "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJyZW56b2xvbGkxQGhvdG1haWwuY29tIiwiZXhwIjoxNzIzMTA1MjU1fQ.R8WslwDOrvGDiqB-CyPPF-m7W_L1WYFWcWKhW-xEY38"
}
```

## Success Response

**Code** : `200 OK`

**Content**

```json
{
  "message": "Valid Token"
}
```

## Unauthorized Response

**Condition** : If token is invalid.

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "message": "Invalid Token"
}
```
