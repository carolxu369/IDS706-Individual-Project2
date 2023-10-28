# IDS706-Individual-Project2: Rust CLI Binary with SQLite

## Description
This project is a command-line interface (CLI) application written in Rust, integrated with an SQLite database. It's designed to demonstrate CRUD (Create, Read, Update, Delete) operations on a SQLite database using Rust's strong type system and safety features. Users can interact with the database by executing various commands through the CLI, which are then processed by the application to perform the corresponding database operations.

## Features
- **CRUD**: Perform Create, Read, Update, and Delete operations on an SQLite database.
- **Rust**: Application coded in Rust, demonstrating its syntax and unique features.
- **Optimized Binary**: The project includes a GitHub Actions workflow that generates an optimized binary for the Rust CLI app.

## Dependencies
- Rust
- SQLite
- Clap
- Rusqlite

To install Rust and Cargo, follow the instructions here: [Install Rust](https://www.rust-lang.org/tools/install).
Install dependencies by running:
  ```
  cargo build
  ```

## How to Run the Program

1. Clone the repository:
   ```sh
   git clone [https://github.com/carolxu369/IDS706-Individual-Project2.git]
   ```
2. Change directory to the project folder:
   ```sh
   cd IDS706-Individual-Project2
   ```

### For Development:
During development, you might prefer a quick compile-run cycle without optimizations. Use the following steps:

1. **Compile and Run**: 
   You can compile and run the application in one step using:
   ```sh
   cargo run -- [command] [arguments]
   ```

Commands available:
- `create [DATA]` - Create a new record with the provided data.
- `read` - Read and display all records.
- `update [ID] [DATA]` - Update an existing record with the provided ID and new data.
- `delete [ID]` - Delete a record by its ID.

For example:
**Execute Commands:**
   - Create a record:
     ```
     cargo run -- create "data to insert"
     ```
   - Read all records:
     ```
     cargo run -- read
     ```
   - Update a record:
     ```
     cargo run -- update 1 "new data"
     ```
   - Delete a record:
     ```
     cargo run -- delete 1

### For Production:
For deployment or distribution, you might want to build an optimized version of your application:

1. **Build the Project in Release Mode**:
   Compile your project with optimizations using:
   ```sh
   cargo build --release
   ```
   This command creates an optimized executable in the `./target/release/` directory.

2. **Execute the Compiled Binary**: 
   Run the binary directly from the terminal:
   - **Create a record**:
     ```sh
     ./target/release/ids706_individual_project2 create "data to insert"
     ```
   - **Read all records**:
     ```sh
     ./target/release/ids706_individual_project2 read
     ```
   - **Update a record**:
     ```sh
     ./target/release/ids706_individual_project2 update 1 "new data"
     ```
   - **Delete a record**:
     ```sh
     ./target/release/ids706_individual_project2 delete 1
     ```

## Using GitHub Copilot
Throughout the development of this project, GitHub Copilot was actively used for:  

1. Code Suggestions: Throughout the development process, GitHub Copilot provided real-time code suggestions. These suggestions were especially helpful in quickly writing code and complex structures, speeding up the development process.
2. CRUD: Guiding the setup of CRUD operations with SQLite.
3. Error Handling: Copilot offered suggestions for effective error handling strategies and Rust's pattern matching features. These suggestions were adapted and integrated to ensure the application runs even when encountering unexpected situations.
4. Efficient Coding: Copilot assists in writing tests and comments for better code understanding and maintenance.
5. New Libraries: Copilot suggested the use of certain Rust crates and libraries, such as `clap` for parsing CLI arguments and `rusqlite` for interacting with the SQLite database.