#!/bin/sh

first_run_flag="/var/first_run_flag"

echo "[*] starting nginx..."
nginx

echo "[*] starting pm2..."
pm2 start ecosystem.config.js

sleep 6

echo "[*] Validating first run..."
if [ ! -f "$first_run_flag" ]; then
  ./create_admin.sh
  touch "$first_run_flag"
else
  echo "[?] Skipping first run..."
fi

echo "[*] logs are available at /data/pm2/*.log"
echo "[:)] all services started!"

tail -f /dev/null
