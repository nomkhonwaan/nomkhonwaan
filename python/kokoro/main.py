"""A small web page that speaks English vocabulary using Kokoro TTS.

Unlike a script that saves .wav files, this runs a tiny local web server.
Each word is synthesized on the fly and streamed to the browser, so nothing
is written to disk and you can click any word to hear it instantly.

Install (one-time):
    brew install espeak-ng            # required for pronunciation
    pip install kokoro torch soundfile

Run:
    python python/kokoro/main.py

Then open http://127.0.0.1:8765 in your browser.
"""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from io import BytesIO
from threading import Lock
from urllib.parse import parse_qs, urlparse

import soundfile as sf
from kokoro import KPipeline

SAMPLE_RATE = 24000

WORDS = [
    "garden", "kitchen", "morning", "winter", "wonderful",
    "mountain", "beautiful", "umbrella", "birthday", "friendship",
    "television", "restaurant", "adventure", "strawberry", "envelope",
    "equation", "community", "brightness", "satisfaction", "underground",
    "conversation",
]

# British English ("b"). For American English use lang_code="a" + an af_*/am_* voice.
LANG = "b"
VOICE = "bm_lewis"
HOST, PORT = "127.0.0.1", 8765

# The model downloads once on startup, then is reused. A lock keeps the
# threaded server from synthesizing two words at the same moment.
pipeline = KPipeline(lang_code=LANG, repo_id="hexgrad/Kokoro-82M")
_lock = Lock()


def synthesize_wav(word: str) -> bytes:
    """Render one word to a WAV byte string (in memory, nothing hits disk)."""
    buf = BytesIO()
    with _lock:
        for _gs, _ps, audio in pipeline(word, voice=VOICE, speed=1.0):
            sf.write(buf, audio.detach().cpu().numpy(), SAMPLE_RATE, format="WAV", subtype="PCM_16")
    return buf.getvalue()


PAGE = """<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Listening practice</title>
  <style>
    body { font-family: sans-serif; max-width: 640px; margin: 40px auto; padding: 0 16px; }
    h1 { font-size: 22px; }
    .grid { display: flex; flex-wrap: wrap; gap: 12px; }
    button {
      font-size: 18px; padding: 14px 20px; border: none; border-radius: 10px;
      background: #1a73e8; color: #fff; cursor: pointer;
    }
    button:hover { background: #1666cc; }
  </style>
</head>
<body>
  <h1>Click a word to hear it</h1>
  <div class="grid" id="grid"></div>
  <audio id="player" hidden></audio>
  <script>
    const words = %WORDS%;
    const grid = document.getElementById("grid");
    const player = document.getElementById("player");
    for (const w of words) {
      const btn = document.createElement("button");
      btn.textContent = w;
      btn.onclick = () => { player.src = "/audio?word=" + encodeURIComponent(w); player.play(); };
      grid.appendChild(btn);
    }
  </script>
</body>
</html>
"""


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        path = urlparse(self.path).path
        if path == "/":
            page = PAGE.replace("%WORDS%", str(WORDS))
            self._send_text(page, "text/html")
        elif path == "/audio":
            word = parse_qs(urlparse(self.path).query).get("word", [""])[0]
            self._send_bytes(synthesize_wav(word), "audio/wav", word)
        else:
            self.send_error(404)

    def _send_text(self, body: str, media: str):
        data = body.encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", media + "; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def _send_bytes(self, data: bytes, media: str, word: str):
        self.send_response(200)
        self.send_header("Content-Type", media)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Content-Disposition", f'inline; filename="{word}.wav"')
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, format, *args):  # keep the console quiet
        pass


def main() -> None:
    print(f"Listening practice: http://{HOST}:{PORT}")
    print(f"Using {VOICE} ({LANG}) - {len(WORDS)} words ready to speak.")
    server = ThreadingHTTPServer((HOST, PORT), Handler)
    server.serve_forever()


if __name__ == "__main__":
    main()