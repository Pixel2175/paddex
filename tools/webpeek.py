import os
import webview
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler
from threading import Thread
from pathlib import Path
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

def run(host, port,md_dir, pages_dir, static_dir=None):
    pages_dir = Path(pages_dir).resolve()
    static_dir = Path(static_dir).resolve() if static_dir else None

    class Handler(SimpleHTTPRequestHandler):
        def translate_path(self, path):
            if static_dir and path.startswith("/static/"):
                return str(static_dir / path[len("/static/"):])
            return str(pages_dir / path.lstrip("/"))

    server = ThreadingHTTPServer((host, port), Handler)
    Thread(target=server.serve_forever, daemon=True).start()
    window = webview.create_window("Paddex", url=f"http://{host}:{port}/")
    class ReloadHandler(FileSystemEventHandler):
        def on_modified(self, event):
            if not event.is_directory:
                print(f"Reloading: {event.src_path}")
                os.system(f"./paddex -f {event.src_path}")
                window.load_url(f"http://{host}:{port}/")

    observer = Observer()
    observer.schedule(ReloadHandler(), path=md_dir, recursive=True)
    observer.start()

    print(f"HOST: {host}\nPORT: {port}")
    webview.start()
