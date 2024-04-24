import os
import requests
from requests.auth import HTTPBasicAuth

# Set the base URL of your FastAPI endpoint
base_url = "https://aidich.pro/protected"  # Replace with your FastAPI endpoint URL

# Load username, password, and activate_beta from config.cfg
with open("config.cfg", "r") as file:
    config = file.read().strip().split(",")
    username = config[0]
    password = config[1]
    activate_beta = config[2].lower() == "true"

# Create the "output" folder if it doesn't exist
output_folder = "output"
if not os.path.exists(output_folder):
    os.makedirs(output_folder)

def split_at_positions(string, num_chunks):
    split_list = string.split('\n')
    chunk_size = len(split_list) // num_chunks
    tach_cau_dau_hieu = '\n'
    if chunk_size == 0:  # khong co dau phay nao
        if '.' in string:
            split_list = string.split('.')
            chunk_size = len(split_list) // num_chunks
            tach_cau_dau_hieu = '.'
        elif '?' in string:
            split_list = string.split('?')
            chunk_size = len(split_list) // num_chunks
            tach_cau_dau_hieu = '?'
        else:
            return split_list
    chunks = []
    for i in range(0, len(split_list), chunk_size):
        chunk = tach_cau_dau_hieu.join(split_list[i:i+chunk_size])
        chunks.append(chunk)
   
    # in case the string does not split evenly into chunks, append remaining items to the last chunk
    if len(split_list) % num_chunks != 0:
        chunks[-2] += tach_cau_dau_hieu + chunks[-1]
        chunks = chunks[:-1]
   
    return chunks

# Get a list of all text files in the current folder
text_files = [file for file in os.listdir(".") if file.endswith(".txt")]

# Process each text file
for file_name in text_files:
    # Read the content of the text file
    with open(file_name, "r", encoding='utf-8') as file:
        content = file.read()
    print(content)

    # Check if the content needs to be split
    if len(list(content)) > 1980:
        chunk_size = max(1, len(list(content)) // 1000)
        list_send_to_api = split_at_positions(content, chunk_size)
    else:
        list_send_to_api = [content]

    # Create the output file for the current text file
    output_file_name = os.path.join(output_folder, file_name)
    with open(output_file_name, "w", encoding='utf-8') as output_file:
        # Process each chunk and append the response to the output file
        for chunk in list_send_to_api:
            # Send the chunk to the FastAPI endpoint
            response = requests.get(
                f"{base_url}",
                params={"message": chunk, "activate_beta": activate_beta},
                auth=HTTPBasicAuth(username, password)
            )
            # Check if the request was successful
            if response.status_code == 200:
                # Extract the response message
                result = response.json()["message"]
                # Append the result to the output file
                output_file.write(result + "\n")
                print(f"Processed chunk and saved the result to {output_file_name}")
            else:
                print(f"Error processing chunk. Status code: {response.status_code}")

    print(f"Processed {file_name} and saved the result to {output_file_name}")