#!/bin/bash

service nginx restart
python3 -m flask --app website run --host 0.0.0.0
