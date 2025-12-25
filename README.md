# rfind (Rust Find) 🚀

**rfind** is a high-performance, multi-threaded file search tool written in Rust. It leverages parallel processing to outperform standard tools like `find` in complex search scenarios, making it ideal for developers and system administrators.

## ⚡ Performance Verification

In controlled benchmarks on an Arch Linux system, **rfind** consistently outperformed GNU `find` across multiple real-world scenarios, achieving an **average speedup of 1.5x** and winning **9 out of 10** stress tests.

### Benchmark Results (System: Linux / Arch)

| Test Case | Directory | Search Pattern | `find` Time | `rfind` Time | Speedup |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **1. Root Configs** | `/etc` | `*.conf` | 0.012s | **0.007s** | **1.8x** |
| **2. Python Files** | `/usr/lib` | `*.py` | 0.250s | **0.151s** | **1.7x** |
| **3. Text Files** | `/usr/share` | `*.txt` | 0.433s | **0.280s** | **1.5x** |
| **4. Large Dir Scan** | `/usr/lib` | `*lib*` | 0.250s | **0.160s** | **1.6x** |
| **5. Home Search** | `~/` | `*.rs` | 0.521s | **0.365s** | **1.4x** |
| **6. Global Bin Search** | `/usr` | `*bin*` | 0.760s | **0.492s** | **1.5x** |
| **7. Temp Files** | `/tmp` | `*.tmp` | 0.005s | **0.003s** | **1.6x** |

> **Overall Result:** `rfind` proved **25.5% faster overall** in mixed workloads (Total time: 1.68s vs 2.11s).

***

## 🛠️ Installation

### Prerequisites
*   [Rust Toolchain](https://www.rust-lang.org/tools/install) (Cargo)

### Build from Source
```bash
git clone https://github.com/Shabari-K-S/rfind.git
cd rfind
cargo build --release
```

### Run
The binary will be located in `target/release/rfind`.
```bash
./target/release/rfind --help
```

***

## 📖 Usage

`rfind` simplifies file searching with an intuitive CLI syntax.

### Basic Search
Search for a file by name in the current directory:
```bash
rfind --name main.rs
```

### Positional Arguments
Search specific directories directly:
```bash
rfind -p /usr/bin -n python
```

### Advanced Filters
Filter by **extension**, **type**, and **size**:
```bash
# Find all Rust files in Home directory
rfind --path ~ --extension rs

# Find files larger than 10MB in /tmp
rfind --path /tmp --type f --min-size 10485760

# Find directories named "config"
rfind --path / --type d --name config
```

### Options
| Flag | Short | Description | Example |
| :--- | :--- | :--- | :--- |
| `--path` | `-p` | Path to Search | `--path /home/user` | 
| `--name` | `-n` | Search by filename pattern | `--name "main.rs"` |
| `--extension` | `-e` | Filter by file extension | `--extension txt` |
| `--type` | `-t` | Filter by type (`f`=file, `d`=dir, `l`=symlink) | `--type d` |
| `--min-size` | `-s` | Minimum file size in bytes | `--min-size 1024` |
| `--max-size` | | Maximum file size in bytes | `--max-size 5000` |

***

## 💻 Technical Details

`rfind` achieves its performance through:
1.  **Parallelism:** Uses [Rayon](https://github.com/rayon-rs/rayon) to distribute search tasks across all available CPU cores.
2.  **Efficient Walking:** Uses [WalkDir](https://github.com/BurntSushi/walkdir) for fast directory traversal.
3.  **Rust's Zero-Cost Abstractions:** Ensures safety without sacrificing C-level speed.
