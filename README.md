# 🌡️ Rust Temperature Converter

A simple, robust, and interactive command-line temperature converter built in Rust. 

This project was built as a hands-on exercise while working through Chapter 3 of *The Rust Programming Language* book. It handles bidirectional conversion between Fahrenheit and Celsius, features input error-recovery loops, and formats decimal outputs to two decimal places.

---

## ✨ Features

- **Bidirectional Conversion:** 
  - Fahrenheit (1) to Celsius (°C)
  - Celsius (2) to Fahrenheit (°F)
- **Robust Error Handling:** Gracefully handles invalid inputs (letters, blank spaces) using Rust's `match` patterns without crashing the application.
- **Interactive Loop:** Runs continuously until the user chooses to exit (`y/n` prompt).
- **Clean Formatting:** Displays converted temperatures precisely rounded to 2 decimal places (`:.2`).

---

## 🚀 Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed on your system. (You can get them via [rustup.rs](https://rustup.rs/)).

### Running the Project

1. Clone your repository:
   ```bash
   git clone [https://github.com/archdevabel/temp_changer.git](https://github.com/archdevabel/temp_changer.git)
   cd temp_changer 
2. Run the application using Cargo:
    cargo run
3. Follow the on-screen prompts in your terminal!
