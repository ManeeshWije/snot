# snot

A tool to search over files via natural language.

# Features

- Search files using natural language queries.
- Index files and their metadata.
- Store file contents, metadata, and embeddings in a vector database.
- Retrieve relevant files based on user queries.

# Installation

```bash
pip install snot
```

# Usage

```python
from snot import Snot
s = Snot()
s.index_files('/path/to/files')  # Index files in the specified directory
results = s.search('find documents about AI')  # Search for files related to AI

for result in results:
    print(result)  # Print the search results
```

# Configuration

You can configure the vector database and other settings in the `snot` configuration file. The default configuration file is located at `~/.snot/config.yaml`. You can modify it to change the database connection settings, indexing options, and more.

# Contributing

We welcome contributions! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

# Contact

For any questions or issues, please open an issue on the GitHub repository or contact the maintainers via email.
