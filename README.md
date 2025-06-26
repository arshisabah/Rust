Rust Practice Programs 🚀


-----------------------------------------------------------------------------------------------------------------------------------------------
Welcome to my Rust programming learning repository! This repo contains a wide range of small Rust programs demonstrating key language features and concepts.
It is an evolving project as I explore more advanced Rust topics.

-----------------------------------------------------------------------------------------------------------------------------------------------
📚 Table of Contents
Repository Structure

Concepts Covered

Setup & Usage

Learning Goals

Planned Next Steps

Contribution

License

-----------------------------------------------------------------------------------------------------------------------------------------------
📂 Repository Structure
pgsql
Copy
Edit
Rust/
├── arrays/

├── basic/

├── collections/

├── enums/

├── error_handling/

├── functions/

├── hashmaps/

├── modules/

├── ownership/

├── strings/

├── structs/

├── traits/

├── vectors/

├── macros/

└── (More folders will be added as I learn)
Each folder contains one or more .rs files demonstrating specific Rust concepts.


-----------------------------------------------------------------------------------------------------------------------------------------------
✅ Concepts Covered
📌 Variables, Data Types, and Control Flow

📌 Ownership and Borrowing

📌 Structs and Enums

📌 Traits and Trait Objects

📌 Modules and Crate Structure

📌 Error Handling (Result, Option, Panic Handling)

📌 Collections: Vectors, HashMaps, Strings

📌 Functions and Closures

📌 Arrays and Slices

📌 Pattern Matching and match Statement

📌 Macros (Basic)

📌 Basic I/O and Standard Library Usage (Planned)


-----------------------------------------------------------------------------------------------------------------------------------------------
🛠️ Setup & Usage
Prerequisites:
Rust (Install Rust)

Basic command-line knowledge

Running Programs:
You can compile and run individual .rs files using:

bash
Copy
Edit
rustc filename.rs
./filename
Or you can structure the entire repository as a Cargo Workspace (see below 👇).


-----------------------------------------------------------------------------------------------------------------------------------------------
🚀 (Optional) Converting This Repo into a Cargo Workspace
To manage everything easily and run with Cargo:

At the root, create a Cargo.toml like this:

toml
Copy
Edit
[workspace]
members = [
    "arrays",
    "basic",
    "collections",
    "enums",
    "error_handling",
    "functions",
    "hashmaps",
    "modules",
    "ownership",
    "strings",
    "structs",
    "traits",
    "vectors",
    "macros"
]
Inside each folder (like arrays/, ownership/, etc.), create individual Cargo projects:

bash
Copy
Edit
cd arrays
cargo init
Now you can build and run each module easily:

bash
Copy
Edit
cargo build --workspace
cargo run -p arrays
Let me know if you want me to generate all the individual Cargo.toml and folder structures for you automatically.


-----------------------------------------------------------------------------------------------------------------------------------------------
🎯 Learning Goals
Gain strong fundamentals in Rust syntax and semantics

Build familiarity with Rust's unique Ownership and Borrowing model

Experiment with modules, traits, collections, and error handling

Prepare for building larger Rust projects (CLI tools, microservices, embedded apps)

🔮 Planned Next Steps
 Add examples on Concurrency (Threads, Channels)

 Practice with File I/O

 Networking (TCP/UDP Sockets)

 Building a small CLI tool

 Working with Rust Crates (like serde, tokio, etc.)

 
-----------------------------------------------------------------------------------------------------------------------------------------------
🤝 Contribution
Feel free to fork the repo and use it for your learning.
If you want to suggest improvements, you can create an Issue or Pull Request.

📜 License
This project is licensed under the MIT License.

⭐️ If this helped you...
Please consider giving the repo a ⭐️ to support my learning journey!
