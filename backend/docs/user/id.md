# Get User By Id

Used to collect first user matched by id.

**URL** : `/api/v1/auth/user?id={id}`

**Method** : `GET`

**Auth required** : YES

## Success Response

**Code** : `200 OK`

**Content**

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

**Content Example**

```json
[
  {
    "username": "user123",
    "password": "P4ssw0rd",
    "email": "usermail@domain.com",
    "couplename": "couple123",
    "anniversary": "17/04/1999",
    "photo": "http://www.freepic.com/love.png"
  }
]
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
