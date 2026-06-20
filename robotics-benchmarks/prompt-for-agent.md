# Prompt for Agent

Read and follow the instructions in the **ROB101_Project** pdf file alongside the local files in the **data/** directory.

## ⚠️ Critical Data Constraint

Do **NOT** attempt to open, parse, or read the raw text contents of these specific massive CSV files (`question_image.csv` or `pointcloud.csv`). They are too large. You ONLY need to understand their file paths and general structure to write the Julia code that handles them locally on disk. You just needs to know that the file exists and contains the LiDAR points. That file contains the actual $x, y, z$ coordinates of the laser scans. Without it, the script will throw a "File Not Found" error and won't be able to draw the 3D map.

## Output Requirements

When completely done, generate exactly two files and place them inside a folder named **solutions**:

1. **solution.jl**: A clean, executable Julia script using relative paths that processes the data to solve both the 2D un-distortion task (`question_image.csv`) and the 3D mapping task (`cassie_data/` frames), displaying the final plots.
2. **SOLUTION.md**: A comprehensive markdown document detailing your engineering approach, the mathematical transformations used, the final results, and any other observations relevant to this benchmark test.
