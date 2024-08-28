# All Date Ideas

<!--toc:start-->

- [All Date Ideas](#all-date-ideas)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Not Found Response](#not-found-response)
  - [Unauthorized Response](#unauthorized-response)
  <!--toc:end-->

## Description

| URL                | Method | Auth required | Description        |
| ------------------ | ------ | ------------- | ------------------ |
| /api/v1/date-idea/ | GET    | YES           | Get all date ideas |

## Success Response

**Code** : `200 OK`

**Content** :

```json
[
  {
    "id": "string",
    "idea": "string",
    "description": "string",
    "tags": [
      {
        "id": "string",
        "name": "string"
      }
    ]
  }
]
```

## Not Found Response

**Code** : `404 NOT FOUND`

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
