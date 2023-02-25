require "csv"
require "json"
require "file_utils"

csv_path = File.join("../data/data.csv")
json_path = File.join("../data/crystal/json")
FileUtils.mkdir_p(json_path)

File.open(csv_path) do |csv_file|
  CSV.each_row(csv_file).with_index do |row, i|
    json_file_path = File.join(json_path, "json#{i}.json")
    File.write(json_file_path, row.to_json)
  end
end
