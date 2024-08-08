# Register

Used to register a new User.

**URL** : `/api/v1/auth/register/`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
  "username": "[username in plain text]",
  "password": "[password in plain text]",
  "email": "[valid email address]",
  "couplename": "[couplename in plain text]",
  "anniversary": "[date in format 'dd/mm/yyyy']",
  "photo": "[valid url pic]"
}
```

**Data example**

```json
{
  "username": "user123",
  "password": "P4ssw0rd",
  "email": "usermail@domain.com",
  "couplename": "couple123",
  "anniversary": "17/04/1999",
  "photo": "http://www.freepic.com/love.png"
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
