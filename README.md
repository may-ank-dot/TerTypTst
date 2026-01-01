
# Rust Typing Test (CLI)

A simple **terminal-based typing speed test** built using **Rust**. The program displays a random sentence, measures how fast and accurately the user types it, and shows the results.

---

## Key Features

* Random sentence selection from a file
* Measures typing time
* Calculates:

  * Words Per Minute (WPM)
  * Accuracy (%)
* Clean and modular Rust code structure

---

## Project Structure

```
typing_test/
├── Cargo.toml
├── Paragraph
└── src/
    ├── main.rs        // Entry point
    ├── sentence.rs    // Sentence loading & selection
    ├── test.rs        // Typing test & timing
    └── stats.rs      // WPM & accuracy calculation
```

---

## How to Run

```bash
cargo run
```

---

## Sentence File Format

`Paragraph` contains comma-separated sentences:

```
Rust is a fast and memory safe programming language that helps developers write reliable software
```

---

## Tech Used

* Rust
* `rand` crate

---

## Author

**Mayank Swaraj**
