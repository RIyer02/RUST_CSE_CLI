# app.py

from flask import Flask, render_template, request
import subprocess
import json

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/process_folder', methods=['POST'])
def process_folder():
    folder_path = request.form['folder_path']
    query = request.form['query']
    url = "https://example.com/your-endpoint"

    # Create a dictionary containing the data
    input_data = {"folder_path": folder_path, "query": query, "url": url}

    # Convert the dictionary to JSON
    json_input = json.dumps(input_data)

    # Run your Rust code as a subprocess
    subprocess.run(['.C:\Users\Dell\Downloads\DesktopSearchEngine-main\DesktopSearchEngine-main\src', json_input])

    return 'Processing complete!'

if __name__ == '__main__':
    app.run(debug=True)
