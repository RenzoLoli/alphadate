#!/bin/sh

nginx

pm2 start ecosystem.config.js
pm2 log
