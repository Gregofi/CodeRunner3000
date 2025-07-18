import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_python(code: str):
    return {"language": "js", "code": code, "executor": "nodejs-bookworm"}

def test_simple_python():
    payload = generate_python("""
console.log("Hello, World!");
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n"
    assert values["stderr"] == ""
