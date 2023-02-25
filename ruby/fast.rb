require "csv"
require "json"
require "fileutils"

csv_path = File.join(__dir__, "../data/data.csv")
json_path = File.join(__dir__, "../data/ruby/json")
FileUtils.mkdir_p(json_path)

File.open(csv_path) do |csv_file|
  CSV.foreach(csv_file).with_index do |row, i|
    json_file_path = File.join(json_path, "json#{i}.json")
    File.write(json_file_path, row.to_json)
  end
end
