echo "Running validate_samplesheet" 
echo "Output is in file: output.txt - Use Wordpad or Notepad++, not notepad" 
@echo off
>output.txt (
  validate_samplesheet.exe -f SampleSheet.csv 
)

timeout 3
