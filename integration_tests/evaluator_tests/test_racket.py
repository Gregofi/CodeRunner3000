import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_racket(code: str):
    return {"language": "racket", "code": code, "executor": "racket-v8.11.1"}

def test_simple_racket():
    payload = generate_racket("""
#lang racket
(display "Hello, World!\\n")
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n"
    assert values["stderr"] == ""
