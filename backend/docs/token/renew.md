# Renew token

<!--toc:start-->

- [Renew token](#renew-token)
  - [Description](#description)
  - [Success Response](#success-response)
  - [Unauthorized Response](#unauthorized-response)
  <!--toc:end-->

## Description

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
