import requests

API_URL = "http://127.0.0.1:8000"

def get_joke_types():
    response = requests.get(f"{API_URL}/jokes/types")
    if response.status_code == 200:
        return response.json()
    return []

def get_random_joke(joke_type=None):
    url = f"{API_URL}/jokes/random"
    if joke_type:
        url += f"?keyword={joke_type}"
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    return None

def main():
    joke_types = get_joke_types()
    if not joke_types:
        print("Could not fetch joke types from the API.")
        return

    while True:
        print("\nSelect a joke type:")
        for idx, jt in enumerate(joke_types, 1):
            print(f"{idx}. {jt.title()}")
        print(f"{len(joke_types)+1}. Exit")

        try:
            choice = int(input("Enter your choice: "))
        except ValueError:
            print("Invalid input. Please enter a number.")
            continue

        if choice == len(joke_types) + 1:
            print("Goodbye!")
            break
        elif 1 <= choice <= len(joke_types):
            selected_type = joke_types[choice - 1]
            joke = get_random_joke(selected_type)
            if joke:
                print(f"\n[{joke['type'].title()} Joke]")
                print(joke['setup'])
                print(joke['punchline'])
            else:
                print("Could not fetch a joke. Try again.")
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()
