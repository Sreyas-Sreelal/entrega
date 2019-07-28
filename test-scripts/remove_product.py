import requests
import json


re = requests.post(
    "http://localhost:8000/product/get_random",
        data=json.dumps({
            'limit':1
}))
print("Response: ",re.json())
id = re.json()["products"][0]["product_id"]

session = requests.Session()
re = session.post(
    "http://localhost:8000/user/auth",
    data=json.dumps({
        'name':'Sreyas',
        'password':'password'
}))
print("Response: ",re.content)
token = re.json()["token"]
print("Token is ",token)

re = requests.post(
    "http://localhost:8000/product/remove",
    data = json.dumps({
        'token':token,
        'data':id
}))