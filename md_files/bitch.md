{% extends "layout.html" %}
{% block title %}Free Gaza{% endblock %}
{% block content %}

# Markdown Showcase

This file shows almost everything you can do in Markdown.

---

## 1. Headings

# H1
## H2
### H3
#### H4
##### H5
###### H6

---

## 2. Emphasis

*Italic* or _Italic_  
**Bold** or __Bold__  
***Bold Italic*** or ___Bold Italic___  
~~Strikethrough~~

---

## 3. Lists
### 3.1 Unordered List
- Item 1
- Item 2
  - Nested Item 2a
  - Nested Item 2b
- Item 3


## 4. Links and Images

[Google](https://www.google.com)  
![Image Alt Text](https://via.placeholder.com/150)

---

## 5. Code

### 5.1 Inline Code
Use `print("Hello")` in Python.

### 5.2 Code Block

```python
def hello():
    print("Hello, World!")
hello()
```

```bash
# Bash example
echo "Hello World"
```

---

## 6. Blockquotes

> This is a blockquote.
>
> > Nested blockquote.

---

## 7. Horizontal Rules

---

***

___

---

## 8. Tables

| Name     | Age | City       |
|----------|-----|------------|
| Alice    | 25  | New York   |
| Bob      | 30  | London     |
| Charlie  | 22  | Tokyo      |

---

## 9. Task Lists

- [x] Task 1
- [ ] Task 2
- [ ] Task 3

---

## 10. Footnotes

Here is a footnote reference[^1].

[^1]: This is the footnote content.

---

## 11. Escaping Characters

Use a backslash to escape Markdown characters: \*Not Italic\*.

---

## 12. HTML in Markdown

You can also use **HTML**:

<p style="color:red;">This is red text using HTML.</p>

---

## 13. Emoji

You can add emoji like this: 😄 🚀 🎉

---

## 14. Definition List (Not standard in GitHub, works in some parsers)

Term 1
: Definition 1

Term 2
: Definition 2

---

## 15. Math (if supported)

Inline math: $E = mc^2$  

Block math:

$$
\int_0^\infty e^{-x} dx = 1
$$

---

## 16. Horizontal Scroll / Code Wrapping

```text
This is a really long line that might wrap in your viewer but shows how text behaves when it goes beyond the width of the screen.
```

---

## 17. Mentioning

Use `@username` to mention someone (depends on platform).

---

## 18. Highlight

==Highlighted text== (works in some Markdown flavors)

---

## 19. Tables with Alignment

| Left Align | Center Align | Right Align |
|:-----------|:------------:|------------:|
| Left       | Center       | Right      |
| Text       | Text         | Text       |

---

## 20. Comments (Hidden in HTML)

[//]: # (This is a comment, invisible in rendered HTML)

---

**End of Markdown Showcase**
{% endblock %}
