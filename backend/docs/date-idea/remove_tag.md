# Remove Tag from Date Idea

<!--toc:start-->

- [Remove Tag from Date Idea](#remove-tag-from-date-idea)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Modified Response](#not-modified-response)
  <!--toc:end-->

## Description

| URL                             | Method | Auth required | Description          |
| ------------------------------- | ------ | ------------- | -------------------- |
| /api/v1/date-idea/:date-idea-id | DELETE | YES           | Remove tag from date |

## URL Params

| Param        | Type   | Description                           |
| ------------ | ------ | ------------------------------------- |
| date-idea-id | String | The id of the date idea to remove tag |

## Request Body

```json
{
  "tag_id": "string"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "id": "string",
  "title": "string",
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

## Not Modified Response

**Code** : `304 NOT MODIFIED`

**Content** :

```json
{
  "message": "string"
}
```
