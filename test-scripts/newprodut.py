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
        'password':'password'
}))
print("Response: ",re.content)
token = re.json()["token"]
print("Token is ",token)

re = session.post(
"http://localhost:8000/product/new",
data=json.dumps({
    'token':'fake',
    'data':{
        'product_name': "fake",
        'price': 1,
        'rating': 1.0,
        'description':"fake"
    }
}),
)
print("Response: ",re.content)

re = session.post(
"http://localhost:8000/product/new",
data=json.dumps({
    'token':'',
    'data':{
        'product_name': "fake",
        'price': 1,
        'rating': 1.0,
        'description':"fake"
    }
}),
)

re = session.post(
"http://localhost:8000/product/new",
data=json.dumps({
    'data':{
        'product_name': "fake",
        'price': 1,
        'rating': 1.0,
        'description':"fake"
    }
}),
)
print("Response: ",re.content)

def genproduct(product_name,price,rating,description):
    global token
    re = session.post(
        "http://localhost:8000/product/new",
        data=json.dumps({
            'token':token,
            'data':{
                'product_name': product_name,
                'price': price,
                'rating': rating,
                'description':description
            }
        }),
        )
    print("Response: ",re.content)


genproduct("Soap",12,7.8,"""
                Soap is the term for a salt of a fatty acid or for a variety of cleansing 
                and lubricating products produced from such a substance. Household uses
                for soaps include washing, bathing, and other types of housekeeping,
                where soaps act as surfactants, emulsifying oils to enable them to 
                be carried away by water.
""")

genproduct("Scrubber",233,1.8,"""
               The exhaust gases of combustion may contain substances considered 
               harmful to the environment, and the scrubber may remove or 
               neutralize those. A wet scrubber is used for cleaning air, 
               fuel gas or other gases of various pollutants and dust particles.
""")

genproduct("Platter",122,72.8,"""
                A platter is a meal or course served on a platter. In restaurant 
                terminology, a platter is often a main dish served on a platter 
                with one or more side dishes, such as a salad or french fries. 
                Notable platters includes the Colombian bandeja paisa, Indian 
                thali or Arabic mixed-meat platters
""")
genproduct("Helmet",122,2.9,"""
               A helmet is a form of protective gear worn to protect the head.
               More specifically, a helmet complements the skull in protecting
               the human brain. Ceremonial or symbolic helmets without protective 
               function are sometimes worn. Soldiers wear helmets, often made from 
               lightweight plastic materials
""")

genproduct("Pen",10,8.1,"""
                A pen is a writing instrument used to apply ink to a surface,
                usually paper, for writing or drawing. Historically, reed pens,
                quill pens, and dip pens were used, with a nib dipped in ink.
""")


