import base64
import json
import os

from dotenv import load_dotenv
from requests import get, post

load_dotenv()

client_id = os.getenv("CLIENT_ID")
client_secret = os.getenv("CLIENT_SECRET")


def get_token():
    authorization_string = f"{client_id}:{client_secret}"
    authorization_bytes = authorization_string.encode("utf-8")
    authorization_base64 = str(base64.b64encode(authorization_bytes), "utf-8")

    url = "https://accounts.spotify.com/api/token"
    headers = {
        "Authorization": f"Basic {authorization_base64}",
        "Content-Type": "application/x-www-form-urlencoded",
    }
    data = {"grant_type": "client_credentials"}

    response = post(url, headers=headers, data=data)
    json_response = json.loads(response.text)

    token = json_response["access_token"]

    return token


def get_authorization_header(token):
    return {"Authorization": f"Bearer {token}"}


def search_for_artist(token, artist_name):
    url = "https://api.spotify.com/v1/search"
    headers = get_authorization_header(token)
    query = f"q={artist_name}&type=artist&limit=1"

    query_url = f"{url}?{query}"

    response = get(query_url, headers=headers)
    json_response = json.loads(response.text)

    artist_items = json_response["artists"]["items"]
    if len(artist_items) == 0:
        print("No artist with this name exists...")
        return None

    return artist_items[0]


def get_songs_by_artist(token, artist_id):
    url = f"https://api.spotify.com/v1/artists/{artist_id}/top-tracks?country=US"
    headers = get_authorization_header(token)
    result = get(url, headers=headers)

    response = get(url, headers=headers)
    json_response = json.loads(response.text)
    track_items = json_response["tracks"]

    return json_response


token = get_token()
result = search_for_artist(token, "ACDC")

artist_id = result["id"]

songs = get_songs_by_artist(token, artist_id)

for index, song in enumerate(songs):
    print(f"{index + 1}. {song['name']}")
