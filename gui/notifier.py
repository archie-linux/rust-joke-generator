import requests
import random
import time
from pync import Notifier

API_URL = "http://127.0.0.1:8000/jokes/random"

def get_random_joke(keyword=None):
    try:
        url = API_URL
        if keyword:
            url += f"?keyword={keyword}"
        resp = requests.get(url)
        if resp.status_code == 200:
            joke = resp.json()
            return f"{joke.get('setup', '')}\n{joke.get('punchline', '')}"
    except Exception as e:
        print("Error fetching joke:", e)
    return None

def main():
    print("Joke notifier started. Press Ctrl+C to stop.")
    keyword = random.choice(["general", "programming", "knock-knock"])
    while True:
        joke = get_random_joke(keyword)
        if joke:
            Notifier.notify(joke, title="Here's your Joke")
        time.sleep(1800)  # 30 minutes

if __name__ == "__main__":
    main()
