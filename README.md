# paddex

A simple static site generator engine.  
It converts Markdown files into HTML pages.

## Build

```bash
make
```

This will compile the tools and place the binaries in `bin/`.

## Config

Edit `config.toml` to set your variables and paths:

```toml
[context]
name = "Alice"
age  = 18

[settings]
layout_file = "./layout.html"
md_dir      = "./src"
pages_dir   = "./pages"
```

- `[context]` — variables available inside your templates
- `[settings]` — paths used by the engine

## Clean

for binaries:
```bash
make clean
```
for static files:
```bash
./paddex --clean
```
