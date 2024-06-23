In this article I’ll show the following:
1. How to get the public key for your AWS Cognito user pool.
2. How to verify a JWT in Python.
3. How to integrate the code into FastAPI to secure a route or a specific endpoint.
4. Bonus: How to extract the username, so that the API handler can work with it.

Background
JSON Web Tokens are represented as an encoded string and contain three parts: The header, the payload/claims, and the signature. The header has information about the algorithm used to sign the token, while additional information like the username is stored in the payload. When a JWT is created–in our case by AWS–the issuer uses a secret key to create the signature. To ensure that no-one tampered with the payload, we have to verify that the signature still matches the payload using the public key. If you’re interested in learning more about JWTs, have a look at JWT.io.

Getting the AWS Cognito public keys
Receiving the public keys is fairly easy once one has dug through the sheer endless AWS documentation. They are saved in a JSON file under the URL:
```
https://cognito-idp.{AWSREGION}.amazonaws.com/{POOLID}/.well-known/jwks.json
```
For example, if your user pool is hosted in Ireland the region is eu-west-1, and your pool id is eu-west-1_PwMfVzLQg, the URL is https://cognito-idp.eu-west-1.amazonaws.com/eu-west-1_PwMfVzLQg/.well-known/jwks.json. If you’re not sure about the pool id, have a look at the pool dashboard in AWS.

Let’s save these values as environmental variables and wrap it into a function:
```
import os
from typing import Dict, List

import requests

JWK = Dict[str, str]
JWKS = Dict[str, List[JWK]]


def get_jwks() -> JWKS:
    return requests.get(
        f"https://cognito-idp.{os.environ.get('COGNITO_REGION')}.amazonaws.com/"
        f"{os.environ.get('COGNITO_POOL_ID')}/.well-known/jwks.json"
    ).json()
```
The format of the jwks.json is:
```
{
  "keys": [
    {
      "alg": "RS256",
      "e": "AQAB",
      "kid": "shorterstring1",
      "kty": "RSA",
      "n": "verylongstring",
      "use": "sig"
    },
    {
      "alg": "RS256",
      "e": "AQAB",
      "kid": "shorterstring2",
      "kty": "RSA",
      "n": "verylongstring",
      "use": "sig"
    }
  ]
}
```
The only important thing to remember is the field kid which represents the key id. AWS issues multiple keys, and we cannot make sure which one they used to sign a JWT. Luckily, the JWT payload tells us the key id we have to use.

Verifying a JWT in Python
We’ll first have to install a new package that deals with all the JWT data: python-jose.

Get the correct public key
Let’s get started by taking our JWT token and find the matching public key based on the key id:
```
from jose import jwt


def get_hmac_key(token: str, jwks: JWKS) -> Optional[JWK]:
    kid = jwt.get_unverified_header(token).get("kid")
    for key in jwks.get("keys", []):
        if key.get("kid") == kid:
            return key
```
The code is pretty straight forward: First, we peek into the header of our token and retrieve the kid that tells us which key was used to create the signature. Then we iterate over through the jwks data to find the matching key.

Verify the JWT
Now that we have our public key, it’s time to verify our token.

First, we’ll convert the JWK-style key into a key object:
```
hmac_key = jwk.construct(get_hmac_key(token, jwks))
```
Next, we’ll have to separate the signature of the JWT from the rest of the token. Since all three parts–header, payload, signature–are separated by a dot, we can use rsplit():
```
message, encoded_signature = token.rsplit(".", 1)
```
In the JWT, the signature is stored as a base64 encoded string; therefore we have to decode it. Note, that the function expects a byte object for the signature, while our encoded_signature variable is a string. We have to encode it as well:
```
decoded_signature = base64url_decode(encoded_signature.encode())
```
Now we’re almost at the finish line! We have our public key in the correct format, the signature is in the right form, and the rest of the token is stored in a separate variable. Again, we first have to encode the string message into a byte object.
```
return hmac_key.verify(message.encode(), decoded_signature)
```
So that’s it! The function returns a boolean that tells us if the JWT is valid or not.

To wrap it all up, here is the code for this part:
```
import os
from typing import Dict, List, Optional

import requests
from jose import jwt, jwk
from jose.utils import base64url_decode

JWK = Dict[str, str]
JWKS = Dict[str, List[JWK]]


def get_jwks() -> JWKS:
    return requests.get(
        f"https://cognito-idp.{os.environ.get('COGNITO_REGION')}.amazonaws.com/"
        f"{os.environ.get('COGNITO_POOL_ID')}/.well-known/jwks.json"
    ).json()


def get_hmac_key(token: str, jwks: JWKS) -> Optional[JWK]:
    kid = jwt.get_unverified_header(token).get("kid")
    for key in jwks.get("keys", []):
        if key.get("kid") == kid:
            return key


def verify_jwt(token: str, jwks: JWKS) -> bool:
    hmac_key = get_hmac_key(token, jwks)

    if not hmac_key:
        raise ValueError("No pubic key found!")

    hmac_key = jwk.construct(get_hmac_key(token, jwks))

    message, encoded_signature = token.rsplit(".", 1)
    decoded_signature = base64url_decode(encoded_signature.encode())

    return hmac_key.verify(message.encode(), decoded_signature)
```
We can use it as follows:
```
jwks = get_jwks()  # Store those once at startup time
# ...
if not verify_jwt(token, jwks):
	  print("You are not verified!")
```
Protecting FastAPI with JWT
Let’s integrate this into our FastAPI app. We can achieve this by defining a dependency. Before the code of a handler is executed, the dependency function is run. If we find anything problematic in the request, we can raise an exception and send an error response right away.

The JWT is sent to the API in the header in the form
```
Authorization: Bearer JWTTOKENeyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzd…
```
To make things easier, FastAPI already ships with a dependency class that can read in the Bearer authorization credentials and catches basic problems (see fastapi/http.py). All we have to do now is to inherit from this class and add our JWT authorization code around it.

First, we have to make sure to use proper pydantic models:
```
from typing import Dict, List

from pydantic import BaseModel


JWK = Dict[str, str]


class JWKS(BaseModel):
    keys: List[JWK]


class JWTAuthorizationCredentials(BaseModel):
    jwt_token: str
    header: Dict[str, str]
    claims: Dict[str, str]
    signature: str
    message: str
```
Now it’s time to construct our dependency itself. The idea is that we initialize the object with the JWKS that we got from our issuer (AWS Cognito in my case). To make it more accessible, we turn it into a dictionary that maps the key id to the public key.

In the main method, we first call the original HTTPBearer which will give us the JWT token from the header. Then we’ll construct a JWTAuthorizationCredentials object and pass it to a second method that verifies it. If everything went smoothly to this point, we can return the JWTAuthorizationCredentials object. If we just use the dependency as a guard to make sure no unauthorized user accesses certain handlers, we don’t have to care about what the method returns — we are just happy it didn’t raise any exceptions. But we can also reuse this method later in case we need to extract information from the JWT.

The whole thing then looks like this:
```
from typing import Dict, Optional, List

from fastapi import HTTPException
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from jose import jwt, jwk, JWTError
from jose.utils import base64url_decode
from pydantic import BaseModel
from starlette.requests import Request
from starlette.status import HTTP_403_FORBIDDEN

JWK = Dict[str, str]


class JWKS(BaseModel):
    keys: List[JWK]


class JWTAuthorizationCredentials(BaseModel):
    jwt_token: str
    header: Dict[str, str]
    claims: Dict[str, str]
    signature: str
    message: str


class JWTBearer(HTTPBearer):
    def __init__(self, jwks: JWKS, auto_error: bool = True):
        super().__init__(auto_error=auto_error)

        self.kid_to_jwk = {jwk["kid"]: jwk for jwk in jwks.keys}

    def verify_jwk_token(self, jwt_credentials: JWTAuthorizationCredentials) -> bool:
        try:
            public_key = self.kid_to_jwk[jwt_credentials.header["kid"]]
        except KeyError:
            raise HTTPException(
                status_code=HTTP_403_FORBIDDEN, detail="JWK public key not found"
            )

        key = jwk.construct(public_key)
        decoded_signature = base64url_decode(jwt_credentials.signature.encode())

        return key.verify(jwt_credentials.message.encode(), decoded_signature)

    async def __call__(self, request: Request) -> Optional[JWTAuthorizationCredentials]:
        credentials: HTTPAuthorizationCredentials = await super().__call__(request)

        if credentials:
            if not credentials.scheme == "Bearer":
                raise HTTPException(
                    status_code=HTTP_403_FORBIDDEN, detail="Wrong authentication method"
                )

            jwt_token = credentials.credentials

            message, signature = jwt_token.rsplit(".", 1)

            try:
                jwt_credentials = JWTAuthorizationCredentials(
                    jwt_token=jwt_token,
                    header=jwt.get_unverified_header(jwt_token),
                    claims=jwt.get_unverified_claims(jwt_token),
                    signature=signature,
                    message=message,
                )
            except JWTError:
                raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="JWK invalid")

            if not self.verify_jwk_token(jwt_credentials):
                raise HTTPException(status_code=HTTP_403_FORBIDDEN, detail="JWK invalid")

            return jwt_credentials
```
To create the JWKS object, we alter the original function a bit:
```
import os

import requests
from app.JWTBearer import JWKS

jwks = JWKS.parse_obj(
    requests.get(
        f"https://cognito-idp.{os.environ.get('COGNITO_REGION')}.amazonaws.com/"
        f"{os.environ.get('COGNITO_POOL_ID')}/.well-known/jwks.json"
    ).json()
)
```
I just put it into a separate file and import the jwks object whenever I need it.

We can now add the dependency to our handler:
```
from fastapi import Depends, FastAPI

from app.JWTBearer import JWTBearer
from app.auth import jwks

app = FastAPI()

auth = JWTBearer(jwks)


@app.get("/secure", dependencies=[Depends(auth)])
async def secure() -> bool:
    return True
```
And that’s it! If you want to secure a whole route, I suggest setting up a FastAPI router, that you add to your app like this:
```
app.include_router(
    user.router,
    prefix="/user",
    dependencies=[Depends(auth)],
)
```
Now, every handler using this router is secured with a JWT token :)

Bonus: Extracting the username from the JWT
In case of AWS Cognito, the username is saved in the JWT payload. That can be pretty useful, as we now don’t have to transfer the name via a GET or URL parameter, or even in our POST body. As it turns out, a handler in FastAPI can directly receive the result of a dependency, if we define it as a parameter to our function. We can simplify matters even more if we wrap this step into a helper function:
```
auth = JWTBearer(jwks)


async def get_current_user(
    credentials: JWTAuthorizationCredentials = Depends(auth)
) -> str:
    try:
        return credentials.claims["username"]
    except KeyError:
        HTTPException(status_code=HTTP_403_FORBIDDEN, detail="Username missing")
```
We can now use it in our handler like this:
```
from fastapi import APIRouter, Depends

from app.auth import get_current_user

router = APIRouter()


@router.get("/test")
async def test(username: str = Depends(get_current_user)):
    return {"username": username}
```
The only drawback here is, that we might authenticate a JWT token twice when we define the JWT dependency for a route and additionally extract the username from the JWT in handlers within this route.
That’s of course not a problem, but it’s not super clean and one might lose 2–3ms, but I don’t think that this should be an actual issue for anyone.

OAuth (with external providers)
In this example, only requests that include a "user" header will be allowed to access the Gradio app. Of course, this does not add much security, since any user can add this header in their request.

Here's a more complete example showing how to add Google OAuth to a Gradio app (assuming you've already created OAuth Credentials on the Google Developer Console):
```
import os
from authlib.integrations.starlette_client import OAuth, OAuthError
from fastapi import FastAPI, Depends, Request
from starlette.config import Config
from starlette.responses import RedirectResponse
from starlette.middleware.sessions import SessionMiddleware
import uvicorn
import gradio as gr

app = FastAPI()

# Replace these with your own OAuth settings
GOOGLE_CLIENT_ID = "..."
GOOGLE_CLIENT_SECRET = "..."
SECRET_KEY = "..."

config_data = {'GOOGLE_CLIENT_ID': GOOGLE_CLIENT_ID, 'GOOGLE_CLIENT_SECRET': GOOGLE_CLIENT_SECRET}
starlette_config = Config(environ=config_data)
oauth = OAuth(starlette_config)
oauth.register(
    name='google',
    server_metadata_url='https://accounts.google.com/.well-known/openid-configuration',
    client_kwargs={'scope': 'openid email profile'},
)

SECRET_KEY = os.environ.get('SECRET_KEY') or "a_very_secret_key"
app.add_middleware(SessionMiddleware, secret_key=SECRET_KEY)

# Dependency to get the current user
def get_user(request: Request):
    user = request.session.get('user')
    if user:
        return user['name']
    return None

@app.get('/')
def public(user: dict = Depends(get_user)):
    if user:
        return RedirectResponse(url='/gradio')
    else:
        return RedirectResponse(url='/login-demo')

@app.route('/logout')
async def logout(request: Request):
    request.session.pop('user', None)
    return RedirectResponse(url='/')

@app.route('/login')
async def login(request: Request):
    redirect_uri = request.url_for('auth')
    # If your app is running on https, you should ensure that the
    # `redirect_uri` is https, e.g. uncomment the following lines:
    # 
    # from urllib.parse import urlparse, urlunparse
    # redirect_uri = urlunparse(urlparse(str(redirect_uri))._replace(scheme='https'))
    return await oauth.google.authorize_redirect(request, redirect_uri)

@app.route('/auth')
async def auth(request: Request):
    try:
        access_token = await oauth.google.authorize_access_token(request)
    except OAuthError:
        return RedirectResponse(url='/')
    request.session['user'] = dict(access_token)["userinfo"]
    return RedirectResponse(url='/')

with gr.Blocks() as login_demo:
    gr.Button("Login", link="/login")

app = gr.mount_gradio_app(app, login_demo, path="/login-demo")

def greet(request: gr.Request):
    return f"Welcome to Gradio, {request.username}"

with gr.Blocks() as main_demo:
    m = gr.Markdown("Welcome to Gradio!")
    gr.Button("Logout", link="/logout")
    main_demo.load(greet, None, m)

app = gr.mount_gradio_app(app, main_demo, path="/gradio", auth_dependency=get_user)

if __name__ == '__main__':
    uvicorn.run(app)
```
