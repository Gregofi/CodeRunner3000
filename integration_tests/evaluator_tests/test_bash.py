import requests

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"


def generate_bash(code: str):
    return {"language": "bash", "code": code}


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
        "stderr": "rm: can't remove '/bin/bash': Permission denied\n",
    },
    {
        "code": "ping 8.8.8.8",
        "stdout": "",
        "stderr": "ping: sendto: Network unreachable\n",
    },
    {
        "code": "ls /evaluator",
        "stdout": "",
        "stderr": "ls: can't open '/evaluator': Permission denied\n",
    },
    {
        "code": "echo 'HI' > /evaluator/file.txt",
        "stdout": "",
        "stderr": "source: line 1: /evaluator/file.txt: Permission denied\n",
    },
    {
        "code": "cat /evaluator/stdin.txt",
        "stdout": "",
        "stderr": "cat: can't open '/evaluator/stdin.txt': Permission denied\n",
    },
    {
        "code": "rm /tmp -rf",
        "stdout": "",
        "stderr": "rm: can't remove '/tmp': Permission denied\n",
    },
    {
        "code": "sudo rm /tmp -rf",
        "stdout": "",
        "stderr": "sudo: a password is required\n",
    },
    {
        "code": "su root -c 'rm /tmp -rf'",
        "stdout": "",
        "stderr": "su: must be suid to work properly\n",
    },
]


# def test_eval_bash_valid_programs():
#     for code in codes:
#         response = requests.post(EVALUATOR_ADDRESS, json=generate_bash(code["code"]))
#         assert response.status_code == 200, code
#         values = response.json()
#         assert values["stdout"] == code["stdout"], code
#         assert values["stderr"].endswith(code["stderr"]), code
