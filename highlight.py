import markdown
from pygments import highlight
from pygments.lexers import get_lexer_by_name
from pygments.formatters import HtmlFormatter
import re
import html as html_module

def highlighter(html_content: str) -> str:
    code_pattern = re.compile(
        r'<code class="language-(\w+)">(.*?)</code>',
        re.DOTALL
    )
    
    matches = list(code_pattern.finditer(html_content))
    
    for match in reversed(matches):
        lang = match.group(1)
        code = html_module.unescape(match.group(2))
        
        try:
            lexer = get_lexer_by_name(lang)
            formatter = HtmlFormatter(
                noclasses=True,
                style='gruvbox-dark',  # Change to: dracula, github-dark, vim, etc.
                nowrap=True
            )
            highlighted = highlight(code, lexer, formatter)
            replacement = highlighted
        except:
            replacement = f'<pre><code>{html_module.escape(code)}</code></pre>'
        
        html_content = html_content[:match.start()] + replacement + html_content[match.end():]
    
    return html_content
