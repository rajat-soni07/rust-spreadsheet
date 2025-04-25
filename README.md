# Rust Spreadsheet Project
The Rust Spreadsheet Project is a comprehensive implementation of a spreadsheet application developed as part of the COP290 course. This project leverages the power and safety of the Rust programming language to deliver a robust and efficient solution for managing and manipulating tabular data.

## Usage
To use the Rust Spreadsheet Project, follow these steps:

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/yourusername/rust_spreadsheet.git
    cd rust_spreadsheet
    ```
2. **Install Dependencies**:  
    Before running the application, install the required dependencies:  
    ```bash
    sudo apt-get update && sudo apt-get install -y libfontconfig1-dev
    ```

3. **Run the Application**:
    Execute the following command to start the spreadsheet application:
    ```bash
    make
    ```
4. **Run the GUI**:  
    To launch the graphical user interface (GUI) of the spreadsheet application, use the following command:  
    ```bash
    make ext1 <nrows> <ncols>
    ```

5. **Testing**:
    Run the test suite to ensure everything works as expected:
    ```bash
    make test
    ```

5. **Coverage**:
    ```bash
    make coverage
    ```

5. **Documentation**:
    ```bash
    make docs
    ```





## Team Members
- Rajat Soni (2023CS10229)
- Krish Bhimani (2023CS10712)
- Priyanshu Gaurav (2023CS10129)