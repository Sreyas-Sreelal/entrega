import requests
import json

re = requests.post(
    "http://localhost:8000/product/get_random",
    data=json.dumps({
        'limit':10
    })
    )
print("Response: ",re.json())