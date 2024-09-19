# Register

<!--toc:start-->

- [Register](#register)
  - [Description](#description)
  - [Data constraints](#data-constraints)
  - [Success Response](#success-response)
  - [Bad Request Response](#bad-request-response)
  - [Conflict Response](#conflict-response)
  <!--toc:end-->

## Description

Used to register a new User.

| URL                    | Method | Auth required | Description |
| ---------------------- | ------ | ------------- | ----------- |
| /api/v1/auth/register/ | POST   | NO            | Register    |

## Data constraints

```json
{
  "username": "[username in plain text]",
  "password": "[password in plain text]",
  "email": "[valid email address]",
  "couplename": "[couplename in plain text]",
  "anniversary": "[date in format 'yyyy/mm/dd']",
  "photo": "[valid url pic]"
}
```

## Success Response

**Code** : `200 OK`

**Content** :

```json
{
  "message": "Registered Succesfully"
}
```

## Bad Request Response

**Condition** : If json combination is wrong.

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "message": "[Cause of the error]"
}
```

## Conflict Response

**Condition** : If register is falied.

**Code** : `409 CONFLICT`

**Content** :

```json
{
  "message": "[Cause of the error]"
}
```
