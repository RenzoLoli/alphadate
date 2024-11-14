#!/bin/bash

address="http://${BACKEND_HOST}:${BACKEND_PORT}/api/v1/auth/register"

echo "[?] Creating admin..."

# curl -X POST "${address}" \
#   -H "Content-Type: application/json" \
#   -d "{\"username\": \"admin123\", \"password\": \"admin123\", \"email\": \"admin@admin.com\", \"couplename\": \"admina\", \"anniversary\": \"2023/01/01\", \"photo\": \"https://img.freepik.com/vector-gratis/ilustracion-pareja-asiatica-dibujada-mano_23-2150016652.jpg\"}"

attempt=0
while true; do
  response=$(curl -o /dev/null -s -w "%{http_code}" -X POST "${address}" \
    -H "Content-Type: application/json" \
    -d "{\"username\": \"admin123\", \"password\": \"admin123\", \"email\": \"admin@admin.com\", \"couplename\": \"admina\", \"anniversary\": \"2023/01/01\", \"photo\": \"https://img.freepik.com/vector-gratis/ilustracion-pareja-asiatica-dibujada-mano_23-2150016652.jpg\"}")

  [ "$response" -eq 200 ] && break
  [ "$attempt" -ge 5 ] && echo "[x] i think something is wrong... try to refresh server!"

  attempt=$((attempt + 1))
  sleep 1
done

echo "[?] Admin created!"
echo "[?] Email: admin@admin.com"
echo "[?] Password: admin123"
