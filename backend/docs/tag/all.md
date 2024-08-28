# All tags

<!--toc:start-->

- [All tags](#all-tags)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL           | Method | Auth required | Description  |
| ------------- | ------ | ------------- | ------------ |
| /api/v1/tags/ | GET    | YES           | Get all tags |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "id": "string",
    "name": "string"
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

## Not Found Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
