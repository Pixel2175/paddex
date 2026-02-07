from json import load

md_dir       = "./md_files"
tools_dir    = "./tools"
pages_dir    = "./pages"
static_dir   = "./static"
template_dir = "./static"
data_file    = "./data.json"

keywords = load(open(data_file,"r"))

filters = [
# replace       thit       ,with this          , strip p?
           ["<p>{% extends",     "{% extends"  , False],
           ["<p>{% block"  ,     "{% block"    , False],
           ["<p><div"      ,     "<div"        , True ],
           ["<p></div"     ,     "</div"       , True ],
]

# Debugging server
host = "0.0.0.0"
port = 8000 
restart_delay = 0

# Options
highlighting = True
