# Clips

*Clips (CLI Password Storage)* is a command-line tool designed for managing local JSON data, ideal for password management. It allows you to search (with support for regular expressions), modify, delete, and generate data entries easily.

## Features

- **Advanced Search:** Use regular expressions to find entries.
- **Modify and Remove:** Efficiently modify or remove entries.
- **Data Generation:** Quickly generate new random data.

## Installation

Requirements:
- git
- rustc
- cargo

```bash
git clone https://github.com/CEKlTA/clips.git
cd clips
cargo b
```

The executable will be located under "clips/target/debug/"
or "clips/target/release" if you used `cargo b --release`

## Usage

```bash
clips <RegEx> [JSON | -r(remove) | -g(generate)]
```

### Search

```bash
clips <RegEx>
clips foo
```

### Modify

```bash
clips <RegEx> [new_value]
clips foo 37
```

### Remove

```bash
clips <RegEx> -r
clips foo -r
```

### Generate

```bash
clips <RegEx> -g
clips foo -g
```

##

## Contributions

Contributions are welcome! If you want to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/new-feature`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add new feature'`).
5. Push to the branch (`git push origin feature/new-feature`).
6. Open a Pull Request.

## License

This project is licensed under the MIT License. (open source)

---

Developed with ❤️ by [cekita](https://github.com/CEKLTA)
