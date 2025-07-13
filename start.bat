@echo off
docker build -t supershort-be .
docker run -p 3000:3000 supershort-be

