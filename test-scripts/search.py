import requests
import json

re = requests.post(
    "http://localhost:8000/product/search",
    data=json.dumps({
        'name':input('input name to search : ')
    })
    )
print("Response: ",re.json())