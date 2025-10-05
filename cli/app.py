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

def int_to_roman(num):
    val = [1, 2, 3, 4]
    syms = ["I", "II", "III", "IV"]
    if 1 <= num <= 4:
        return syms[num - 1]
    return str(num)

def main():
    joke_types = get_joke_types()
    if not joke_types:
        print("Could not fetch joke types from the API.")
        return

    # Only allow up to 4 choices
    joke_types = joke_types[:4]
    option_romans = [int_to_roman(i + 1) for i in range(len(joke_types))]
    exit_roman = int_to_roman(len(joke_types) + 1)

    while True:
        print("\nSelect a joke type:")
        for idx, jt in enumerate(joke_types):
            print(f"{option_romans[idx]}. {jt.title()}")
        print(f"{exit_roman}. Exit")

        choice = input("Enter your choice (I, II, III, IV): ").strip().upper()

        if choice == exit_roman:
            print("Goodbye!")
            break
        elif choice in option_romans:
            selected_type = joke_types[option_romans.index(choice)]
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
