import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"

def generate_haskell(code: str):
    return {"language": "haskell", "code": code, "compiler": "ghc-bookworm"}

def test_simple_haskell():
    payload = generate_haskell("""
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

main = putStrLn $ show $ factorial 5
""")
    response = requests.post(EVALUATOR_ADDRESS, json=payload, headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 200
    values = response.json()
    assert values["stdout"] == "120\n", values['stderr']
    assert values["stderr"] == ""
