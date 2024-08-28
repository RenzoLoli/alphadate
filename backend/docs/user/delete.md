# Delete User

| URL                   | Method | Auth required | Description       |
| --------------------- | ------ | ------------- | ----------------- |
| /api/v1/auth/user/:id | DELETE | YES           | Delete user by id |

## URL Params

| Param | Type   | Description                  |
| ----- | ------ | ---------------------------- |
| id    | String | The id of the user to delete |

## Success Response

**Code** : `200 OK`

**Content** :

```json
  {
    "id": "string"
    "username": "string",
    "email": "string",
    "couplename": "string",
    "anniversary": "string",
    "photo": "string"
  }
```

## Error Response

**Code** : `404 NOT FOUND`

**Content** :

```json
{
  "message": "string"
}
```
