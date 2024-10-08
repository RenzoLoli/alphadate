# Login

<!--toc:start-->

- [Login](#login)
  - [Description](#description)
  - [Data constraints](#data-constraints)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Not Found Response](#not-found-response)
  <!--toc:end-->

## Description

Used to collect a Token for a registered User.

| URL                 | Method | Auth required | Description |
| ------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/login/ | POST   | NO            | Login       |

## Data constraints

```json
{
  "email": "[valid email address]",
  "password": "[password in plain text]"
}
```

## Success Response

**Code** : `200 OK`

**Content**

```json
{
  "token": "[user token]"
}
```

**Content Example**

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJyZW56b2xvbGkxQGhvdG1haWwuY29tIiwiZXhwIjoxNzIzMTA1MjU1fQ.R8WslwDOrvGDiqB-CyPPF-m7W_L1WYFWcWKhW-xEY38"
}
```

## Bad Request Response

**Condition** : If query input is wrong.

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "[Cause of the error]"
}
```

## Not Found Response

**Condition** : If user is not found.

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "[Cause of the error]"
}
```
