# Resources
https://serverjars.com/
https://centrojars.com/
https://adoptium.com/

## Libraries



## Tutorials



## Tools
Python Script that does the heavy lifting of download and generating a   
usable csv file for all the available versions from adoptium.

```python
import requests
import csv
response = requests.get('https://centrojars.com/api/fetchJar/fetchAllTypes.php')
types_data = response.json()
all_files = []
for type_key, subtypes in types_data['response'].items():
    for subtype in subtypes:
        details_url = f'https://centrojars.com/api/fetchJar/{type_key}/{subtype}/fetchAllDetails.php'
        details_response = requests.get(details_url)
        details_data = details_response.json()

        if details_data['status'] == 'success':
            for file_info in details_data['response']['files']:
                file_entry = {
                    'Type': type_key,
                    'Subtype': subtype,
                    'Version': file_info['version'],
                    'File': file_info['file'],
                    'Size (Display)': file_info['size']['display'],
                    'Size (Bytes)': file_info['size']['bytes'],
                    'MD5': file_info['md5'],
                    'Built': file_info['built'],
                    'Stability': file_info['stability']
                }
                all_files.append(file_entry)
csv_file = 'jar_files.csv'
csv_columns = ['Type', 'Subtype', 'Version', 'File', 'Size (Display)', 'Size (Bytes)', 'MD5', 'Built', 'Stability']

try:
    with open(csv_file, 'w', newline='', encoding='utf-8') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=csv_columns)
        writer.writeheader()
        for file_data in all_files:
            writer.writerow(file_data)
    print(f"Data successfully written to {csv_file}")
except IOError as e:
    print(f"I/O error({e.errno}): {e.strerror}")
```

## Documentation
https://serverjars.com/documentation

# BTW I AM A FUCKING GOD
