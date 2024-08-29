# Remove Idea from Alphabet

<!--toc:start-->

- [Remove Idea from Alphabet](#remove-idea-from-alphabet)
  - [Description](#description)
  - [URL Params](#url-params)
  - [Request Body](#request-body)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Unauthorized Response](#unauthorized-response)
  - [Not Modified Response](#not-modified-response)
  <!--toc:end-->

## Description

| URL                           | Method | Auth required | Description               |
| ----------------------------- | ------ | ------------- | ------------------------- |
| /api/v1/alphabet/:alphabet-id | DELETE | YES           | Remove idea from alphabet |

## URL Params

| Param       | Type   | Description                                |
| ----------- | ------ | ------------------------------------------ |
| alphabet-id | String | The id of the alphabet to remove idea from |

## Request Body

```json
{
  "date_idea_id": "string"
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
