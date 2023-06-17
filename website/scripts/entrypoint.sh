#!/bin/bash

service nginx restart
flask --app website run --host 0.0.0.0
