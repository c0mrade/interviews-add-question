# Interview Question Helper Utility

This utility is a Rust application designed to streamline the process of adding new interview questions to the [c0mrade/interviews](https://github.com/c0mrade/interviews) repository. It automates the creation of directories and files according to the specifications of the repository, ensuring consistency and reducing manual effort.

## Overview

When adding a new question to the interview repository, maintaining a consistent structure is crucial for organization and accessibility. This utility prompts the user for the necessary details (e.g., question name, programming language) and automatically sets up the directory structure and required files.

## Features

- **Prompt for Question Details**: The utility asks for the question name and the programming language.
- **Automatic Directory Creation**: Based on the input, it creates a structured directory path `question_name/solutions/language/01`.
- **File Creation**: It generates a `README.md` in the main question directory and `.keep` files in the necessary subdirectories to maintain the structure in Git.

## Getting Started

### Prerequisites

- Rust programming environment (see [Rust installation](https://www.rust-lang.org/tools/install))
- Git installed on your system

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/c0mrade/interviews-add-question.git
   cd interviews-add-question
