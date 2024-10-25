# Update Tag

<!--toc:start-->

- [Update Tag](#update-tag)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Bad Request Response](#bad-request-response)
  - [Internal Server Error Response](#internal-server-error-response)
  <!--toc:end-->

## Description

| URL             | Method | Auth required | Description |
| --------------- | ------ | ------------- | ----------- |
| /api/v1/tag/:id | PUT    | YES           | Update tag  |

## URL Params

| Param | Type   | Description                 |
| ----- | ------ | --------------------------- |
| id    | String | The id of the tag to update |

## Request Body

```json
{
  "name": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "name": "string"
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

## Internal Server Error Response

**Code** : `500 INTERNAL SERVER ERROR`

**Content** :

```json
{
  "message": "string"
}
```
