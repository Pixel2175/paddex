# paddex

A simple static site generator engine.  
It converts Markdown files into HTML pages.

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/pix66)

## Build

```bash
make
```

Binaries are placed in `bin/`.

## Config

```toml
[context]
name = "Alice"
age  = 18

[settings]
layout_file = "./layout.html"
md_dir      = "./src"
pages_dir   = "./pages"
```

- `[context]`  — template variables
- `[settings]` — engine paths

## Clean

```bash
make clean        # binaries
./paddex --clean  # static files
```

---

For full documentation — templates, layouts, loops, conditionals and more — visit **[pi66.xyz/tools/paddex](https://pi66.xyz/tools/paddex)**.
