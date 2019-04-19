import requests
import json

re = requests.post(
    "http://localhost:8000/product/new",
    data=json.dumps({
        'product_name': "Helmet",
        'price': 300,
        'rating': 7.0,
    })
    )
print("Response: ",re.content)

session = requests.Session()
re = session.post(
    "http://localhost:8000/user/auth",
    data=json.dumps({
        'name':'Sreyas',
        'password':'mypass'
}))

print("Response: ",re.content)
#print("\n Session cookies ",re.cookies)
token = re.cookies.get("jwt")
print("Token is ",token)

re = session.post(
    "http://localhost:8000/product/new",
    data=json.dumps({
        'product_name': input("Enter product_name: "),
        'price': int(input("Enter price: ")),
        'rating': float(input("Enter rating: ")),
    }),
    cookies={"jwt":token}
    )
print("Response: ",re.content)

