# fi
`fi` is a CLI tool for summarizing the contents of directories. It scans directories to count files by extension and calculate their cumulative size, simplifying file management and storage analysis.

## Usage
To use fi, navigate to your target directory and execute:
```shell
fi ./path/to/directory
```

This command outputs a table with the count and size of files by type, providing a quick overview of directory contents.
### Example Output
```
⢺ Scanning files...
+-----------+-------+----------+
| Extension | Count | Size     |
+-----------+-------+----------+
|    jpg    |   205 | 16.67 MB |
|    glb    |    33 | 42.19 MB |
|    png    |    30 |  1.63 MB |
...
|           |   297 | 68.06 MB |
+-----------+-------+----------+
```

## Installation
```shell
N/A
```
