import requests
import random
import string


def generate_random_string(length: int):
    letters = string.ascii_letters
    result_str = ''.join(random.choice(letters) for _ in range(length))
    return result_str


random_string = generate_random_string(10)
print(random_string)

EVALUATOR_ADDRESS = "http://evaluator:7800/api/v1/evaluate"
EVALUATOR_METRICS = "http://evaluator:7800/metrics"
NEW_LINK = "http://evaluator:7800/api/v1/link/new"
GET_LINK = "http://evaluator:7800/api/v1/link/get/"

def test_links():
    # 20 is the allowed burst by rate limiter
    for i in range(20):
        # random string
        data = generate_random_string(10)
        response = requests.post(NEW_LINK, data=data, headers={"X-Forwarded-For": "1.1.1.1"})
        assert response.status_code == 200, f"try {i}"

        key = response.json()["key"]
        
        response = requests.get(GET_LINK + key)
        assert response.status_code == 200
        returned_data = response.json()["value"]
        assert returned_data == data
    
    response = requests.post(NEW_LINK, data="test", headers={"X-Forwarded-For": "1.1.1.1"})
    assert response.status_code == 429

    response = requests.post(NEW_LINK, data="test", headers={"X-Forwarded-For": "2.1.1.1"})
    assert response.status_code == 200
