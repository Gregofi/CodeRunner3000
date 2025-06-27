import requests
from time import sleep

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"
EVALUATOR_METRICS = "http://evaluator:7800/metrics"


def generate_lua(code: str):
    return {"language": "lua", "code": code, "executor": "lua5.1.5"}


def test_eval_lua_basic():
    code = "print('Hello, World!')"
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == 'Hello, World!\n'
    assert values["stderr"] == ''

    metrics = requests.get(EVALUATOR_METRICS).text
    assert 'evaluator_requests_by_language{language="lua"}' in metrics


def test_eval_lua_funs():
    code = """
function fact(n)
    if n == 0 then
        return 1
    else
        return n * fact(n - 1)
    end
end

print(fact(5))
print(fact(0))
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == '120\n1\n'
    assert values["stderr"] == ''


def test_eval_runtime_error():
    code = """
print(x())
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert "attempt to call global 'x' (a nil value)" in values["stderr"] 

def test_eval_syntax_error():
    code = """
fun fact(x) return x end
print(fact(5))
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert values["stderr"].endswith("'=' expected near 'fact'\n")


def test_eval_timeout1():
    code = """
while 1 < 2 do end
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert values["stderr"].endswith("error: program exited with non-zero exit code 137 (timed out)\n")


def test_files():
    code = """
local file, err = io.open("/etc/shadow", "r")
print(err)
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    # An useless test because /etc/shadow is not created, but it
    # may be in the future and then it shouldn't be readable.
    assert values["stdout"].endswith('No such file or directory\n')
    assert values["stderr"] == ''
