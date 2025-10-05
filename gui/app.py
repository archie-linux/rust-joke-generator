from flask import Flask, render_template, redirect, url_for, request, session
from flask_bootstrap import Bootstrap
import requests

app = Flask(__name__)
app.secret_key = "super_secret_key"
Bootstrap(app)

def fetch_random_joke(keyword=None):
    url = 'http://127.0.0.1:8000/jokes/random'
    if keyword:
        url += f'?keyword={keyword}'
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    return None

def fetch_joke_types():
    response = requests.get('http://127.0.0.1:8000/jokes/types')
    if response.status_code == 200:
        return response.json()
    return []

@app.route('/')
def index():
    joke_types = fetch_joke_types()
    jokes = session.get("jokes", [])
    selected_type = session.get("selected_type", "Any")
    return render_template('index.html', jokes=jokes, joke_types=joke_types, selected_type=selected_type)

@app.route('/get_joke', methods=['POST'])
def get_joke():
    joke_type = request.form.get('joke_type')
    selected_type = joke_type if joke_type else "Any"
    keyword = joke_type if joke_type and joke_type != "Any" else None
    print(f"[LOG] /get_joke called with joke_type='{joke_type}', keyword='{keyword}'")
    joke = fetch_random_joke(keyword)
    print(f"[LOG] Joke fetched: {joke}")
    jokes = session.get("jokes", [])
    if joke:
        # Normalize joke_type for template compatibility
        if "joke_type" not in joke and "type" in joke:
            joke["joke_type"] = joke["type"]
        jokes.append(joke)
    session["jokes"] = jokes
    session["selected_type"] = selected_type
    return redirect(url_for('index'))

@app.route('/clear_jokes', methods=['POST'])
def clear_jokes():
    session["jokes"] = []
    return redirect(url_for('index'))

if __name__ == '__main__':
    app.run(debug=True, port=5000)
