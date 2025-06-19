# GRIT

**G**eneral **R**ust **I**nterface **T**ool — A do-it-all CLI built with Rust.

**GRIT** is a modular, general-purpose command-line tool written in Rust, designed to provide a variety of small but powerful utilities under one interface. It's intended to grow into a flexible toolbox of developer-friendly commands.

> But it's mostly a personal project built for personal needs :)

---

## 🚀 Installation

### From source

Make sure you have Rust installed. Then run:

```bash
cargo install --path .
````

Or clone and install manually:

```bash
git clone https://github.com/SafetImamovic/GRIT
cd grit
cargo install --path .
```

---

## 📝 Usage

```bash
grit <command>
```

### Examples

```bash
grit --help
```
**Output:**
```bash
  .g8"""bgd `7MM"""Mq.  `7MMF'MMP""MM""YMM
.dP'     `M   MM   `MM.   MM  P'   MM   `7
dM'       `   MM   ,M9    MM       MM
MM            MMmmdM9     MM       MM
MM.    `7MMF' MM  YM.     MM       MM
`Mb.     MM   MM   `Mb.   MM       MM
  `"bmmmdPY .JMML. .JMM..JMML.   .JMML.

  General      Rust    Interface  Tool

multi-purpose CLI utility written in Rust
(ASCII art generated @ https://www.patorjk.com/software/taag/
[font: Georgia11])


Usage: grit.exe <COMMAND>

Commands:
  pwd   Prints the current working directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
grit pwd
```

**Output:**

```
/home/youruser/projects/grit
```

---



## 🔧 Features

- ✅ `grit pwd`

Prints the current working directory as a single clean line — just like `pwd` on Unix-based systems.

#### ✅ Why this exists

In PowerShell, running the built-in `Get-Location` (or its alias `pwd`) returns output in a *formatted object* table:

```powershell
PS > pwd

Path
----
C:\Users\Safet\Projects\grit
```

This format is not ideal when you're scripting or trying to pipe it into something like `clip.exe`. For example, to get just the path as plain text, you'd need to do something awkward like:

```powershell
(pwd).Path | Select-Object -Last 1 | Set-Clipboard
```

Or even more verbose:

```powershell
(Get-Location).Path | ForEach-Object { $_ } | clip.exe
```

This is frustrating when you're used to the Unix (More like `WSL`) experience of:

```bash
pwd | clip.exe
```

With `grit pwd`, you get consistent, cross-platform output — no table formatting, no extra properties — just the raw path.

#### 📦 Example

```powershell
PS > grit pwd | clip.exe
```

That’s it. Clean and pipeable. No weird formatting. No extra parsing.

---

Let me know if you’d like to add similar Unix-friendly fixes or pipe-safe utilities to GRIT — it’s a great theme for building small but impactful tools.


More functionality will be added over time, each one designed to be intuitive and easily accessible through subcommands.

---

## 📦 Planned Commands

* `grit find <pattern>` – A fast file search tool
* `grit sysinfo` – Show basic system information
* `grit uuid` – Generate a UUID
* `grit hash <file>` – Hash a file using SHA algorithms
* ...and more

---

## 🤝 Contributing

Got a neat idea or small utility that would fit? Feel free to open a PR or issue. GRIT is designed to be modular, making it easy to add new subcommands.

---

## 📄 License

MIT License © 2025 \[@SafetImamovic]

---

## 💡 Inspiration

Like its name suggests, **GRIT** is about resilience and utility — a tool that grows with you and your workflow.


