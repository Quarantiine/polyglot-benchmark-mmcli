# Prompt for Agent

Read and follow the instructions in the **ROB101_Project_1.pdf** file alongside the local files in this workspace.

## 🔍 Critical Context Dependency

The provided PDF is a supplement. The actual initial coordinate matrices (like the 2D green square) and the specific transformation matrices for Task 2 reside inside the Jupyter Notebook (**Project01.ipynb**). You MUST inspect the code/markdown cells of `Project01.ipynb` to extract these values and equations.

Additionally:

- Do not write a data parser from scratch. Locate the pre-written `data_parser` function inside the notebook cells and reuse or adapt its logic in your final script to load the data frames.

## ⚠️ Critical Data Constraint

Do **NOT** attempt to open, parse, or read the raw text contents of the massive data CSV files (such as `question_image.csv` or any `pointcloud.csv`) directly into your context. They are too large. You ONLY need to understand their file paths and structural layout to write the Julia code that processes them locally on disk. You just need to know that these files exist and contain the LiDAR point coordinate data.

## Output Requirements

When completely done, generate exactly two files and place them inside a folder named **solutions**:

1. **solution.jl**: A clean, executable Julia script using relative paths that processes the data to solve both the 2D un-distortion task and the 3D mapping task, displaying or saving the final plots.
2. **SOLUTION.md**: A comprehensive markdown document detailing your engineering approach, the mathematical transformations used (including the 2D affine steps and 3D coordinate frame alignments), the final results, and any observations.
