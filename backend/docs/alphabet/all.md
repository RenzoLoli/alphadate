# All Alphabets

<!--toc:start-->

- [All Alphabets](#all-alphabets)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  <!--toc:end-->

## Description

| URL                        | Method | Auth required | Description       |
| -------------------------- | ------ | ------------- | ----------------- |
| /api/v1/alphabets/:user_id | GET    | YES           | Get all alphabets |

## URL Params

| Param   | Type   | Description                             |
| ------- | ------ | --------------------------------------- |
| user_id | String | The id of the user to get alphabets for |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "name": "string",
    "description": "string",
    "is_active": "string",
    "created_at": "string",
    "updated_at": "string"
  }
]
```

## Unauthorized Response

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "message": "string"
}
```
