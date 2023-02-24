
import csv
import json
 
def make_json(csvFilePath):
     
    csv_file = open(csvFilePath, "r")
    csv_reader = csv.DictReader(csv_file)
    row = 0
    for lines in csv_reader:
        output = json.dumps(lines)
        output_json = open("../data/python/json"+str(row)+".json", "w")
        output_json.write(output)
        row += 1
    output_json.close()
    csv_file.close()

csvFilePath = r'../data/data.csv'
make_json(csvFilePath)