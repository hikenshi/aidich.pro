import os
import requests
from requests.auth import HTTPBasicAuth

# Set the base URL of your FastAPI endpoint
base_url = "https://aidich.pro/protected"  # Replace with your FastAPI endpoint URL
# Load username and password for authentication
with open("config.cfg", "r") as file:
    user_info = file.read()
    username, password = user_info.strip().split(",")

# Create the "output" folder if it doesn't exist
output_folder = "output"
if not os.path.exists(output_folder):
    os.makedirs(output_folder)

# Get a list of all text files in the current folder
text_files = [file for file in os.listdir(".") if file.endswith(".txt")]

# Process each text file
for file_name in text_files:
    # Read the content of the text file
    with open(file_name, "r", encoding='utf-8') as file:
        content = file.read()
    print(content)
    # Send the content to the FastAPI endpoint
    response = requests.get(
        f"{base_url}",
        params={"message": content},
        auth=HTTPBasicAuth(username, password)
    )

    # Check if the request was successful
    if response.status_code == 200:
        # Extract the response message
        result = response.json()["message"]

        # Write the result to a file in the "output" folder
        output_file_name = os.path.join(output_folder, file_name)
        with open(output_file_name, "w", encoding='utf-8') as output_file:
            output_file.write(result)
        print(f"Processed {file_name} and saved the result to {output_file_name}")
    else:
        print(f"Error processing {file_name}. Status code: {response.status_code}")