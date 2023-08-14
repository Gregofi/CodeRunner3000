import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/run"


def generate_lua(code: str):
    return {"language": "Lua", "code": code}


def test_eval_lua_basic():
    code = "print('Hello, World!')"
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == 'Hello, World!\n'
    assert values["stderr"] == ''


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
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == '120\n1\n'
    assert values["stderr"] == ''


def test_eval_runtime_error():
    code = """
print(x())
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert values["stderr"] == 'source.lua:2: attempt to call global \'x\' (a nil value)'


def test_eval_syntax_error():
    code = """
fun fact(x) return x end
print(fact(5))
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert values["stderr"] == "source.lua:2: '=' expected near 'fact'"


def test_eval_timeout1():
    code = """
while 1 < 2 do end
"""
    payload = generate_lua(code)
    response = requests.post(EVALUATOR_ADDRESS, json=payload)
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == ''
    assert values["stderr"] == "The program timeouted\n"
