import requests
import json

re = requests.post(
    "http://localhost:8000/user/register",
    data=json.dumps({
        'user_name':'Sreyas',
        'display_name':'SyS',
        'password':'mypass',
        'email':'sreyassreelal@gmail.com',
        'address':'Gayathri Bhavan Chirakadavu East PO Ponkunnam'
    })
    )
print("Response: ",re.content)

