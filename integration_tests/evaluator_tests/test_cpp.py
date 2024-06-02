import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_cpp(code: str):
    return {"language": "cpp", "code": code, "compiler": "gcc-bookworm"}

def test_simple_cpp():
    payload = generate_cpp("""
#include <iostream>                        

int main() {
    std::cout << "Hello, World!\\n";
    return 0;
}
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n", values['stderr']
    assert values["stderr"] == ""


def test_empty_args():
    payload = generate_cpp("""
#include <iostream>                        

int main() {
    std::cout << "Hello, World!\\n";
    return 0;
}
""")

    payload.compiler_args = ['']
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "Hello, World!\n", values['stderr']
    assert values["stderr"] == ""
