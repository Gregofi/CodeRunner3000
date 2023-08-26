#!/bin/bash

service nginx restart
gunicorn -w 4 website:app
