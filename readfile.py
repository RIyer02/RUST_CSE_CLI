from flask import Flask, render_template, request
import subprocess

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/process_folder', methods=['POST'])
def process_folder():
    folder_path = request.form['folder_path']
    url = "https://example.com/your-endpoint"

    # Run your Rust code as a subprocess
    subprocess.run(['.C:\Users\Dell\Downloads\DesktopSearchEngine-main\DesktopSearchEngine-main\src', folder_path, url])

    return 'Processing complete!'

if __name__ == '__main__':
    app.run(debug=True)
