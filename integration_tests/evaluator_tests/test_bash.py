import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"


def generate_bash(code: str):
    return {"language": "bash", "code": code, "executor": "bash-bookworm"}


codes = [
    {
        "code": "echo 'Hello, World!'",
        "stdout": "Hello, World!\n",
        "stderr": "",
    },
    {
        "code": "echo 'Hi' > file.txt; cat file.txt",
        "stdout": "Hi\n",
        "stderr": "",
    },
    {
        "code": "rm -rf /bin/bash",
        "stdout": "",
        "stderr": "rm: cannot remove '/bin/bash': Read-only file system\n",
    },
    {
        "code": "ls /opt/evaluator/sources/bash | wc -l",
        "stdout": "1\n", 
        "stderr": "",
    },
    {
        "code": "echo 'HI' > /evaluator/file.txt",
        "stdout": "",
        "stderr": "error: program exited with non-zero exit code",
    },
]


def test_eval_bash_valid_programs():
    for code in codes:
        response = requests.post(EVALUATOR_ADDRESS, json=generate_bash(code["code"]))
        assert response.status_code == 200, code
        values = response.json()
        assert values["stdout"] == code["stdout"]
        assert code["stderr"] in values["stderr"]
