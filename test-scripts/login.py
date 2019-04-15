import requests
import json

re = requests.post(
    "http://localhost:8000/user/auth",
    data=json.dumps({
        'name':'Sreyas',
        'password':'mypass'
    }))

print("Response: ",re.content)

re = requests.post(
    "http://localhost:8000/user/auth",
    data=json.dumps({
        'name':'Ssreyas',
        'password':'mypass'
    }))

print("Response: ",re.content)

re = requests.post(
    "http://localhost:8000/user/auth",
    data=json.dumps({
        'name':'Sreyas',
        'password':'myspass'
    }))

print("Response: ",re.content)
