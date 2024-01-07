import requests

EVALUATOR_METRICS = "http://evaluator:7800/metrics"

def pytest_sessionstart(session):
    # for some reason, the metrics need to be woken up.
    requests.get(EVALUATOR_METRICS)
