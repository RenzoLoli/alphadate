# Delete Tag

<!--toc:start-->

- [Delete Tag](#delete-tag)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Bad Request Response](#bad-request-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

| URL             | Method | Auth required | Description |
| --------------- | ------ | ------------- | ----------- |
| /api/v1/tag/:id | DELETE | YES           | Delete tag  |

## URL Params

| Param | Type   | Description                 |
| ----- | ------ | --------------------------- |
| id    | String | The id of the tag to delete |

## Success Response

**Code** : `200 OK`

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

## Bad Request Response

**Code** : `400 BAD REQUEST`

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
