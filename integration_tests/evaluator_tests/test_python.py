import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_python(code: str):
    return {"language": "python3", "code": code, "executor": "python3-bookworm"}

def test_simple_python():
    payload = generate_python("""
print("Hello, World!")
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n"
    assert values["stderr"] == ""
