import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_c(code: str):
    return {"language": "c", "code": code, "compiler": "gcc-bookworm"}

def test_simple_c():
    payload = generate_c("""
#include <stdio.h>

int main() {
    printf("Hello, World!\\n");
    return 0;
}
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n", values['stderr']
    assert values["stderr"] == ""

