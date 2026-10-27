"""Serve candidate files only on an ephemeral loopback socket for install checks."""
from contextlib import contextmanager
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass


@contextmanager
def server(directory):
    http = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(directory)))
    thread = Thread(target=http.serve_forever, daemon=True)
    thread.start()
    try:
        yield "http://127.0.0.1:" + str(http.server_port)
    finally:
        http.shutdown()
        thread.join()
        http.server_close()



@contextmanager
def https_server(directory, openssl="openssl"):
    """Serve an owned fixture over TLS with a temporary, fixture-only test CA."""
    import os
    from pathlib import Path
    import ssl
    import subprocess
    import tempfile
    with tempfile.TemporaryDirectory(prefix="installer-tls-") as temporary:
        root = Path(temporary).resolve()
        cert, key, config = root / "ca.pem", root / "key.pem", root / "openssl.cnf"
        config.write_text("[req]\ndistinguished_name=dn\nx509_extensions=ext\nprompt=no\n[dn]\nCN=localhost\n[ext]\nsubjectAltName=IP:127.0.0.1\nbasicConstraints=critical,CA:TRUE\n")
        env = {"PATH": os.defpath, "HOME": str(root), "OPENSSL_CONF": os.devnull}
        if "SystemRoot" in os.environ:
            env["SystemRoot"] = os.environ["SystemRoot"]
        subprocess.run([openssl, "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-days", "1",
                        "-config", str(config),
                        "-keyout", str(key), "-out", str(cert)], env=env, check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        http = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(directory)))
        context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        context.load_cert_chain(cert, key)
        http.socket = context.wrap_socket(http.socket, server_side=True)
        thread = Thread(target=http.serve_forever, daemon=True)
        thread.start()
        try:
            yield "https://127.0.0.1:" + str(http.server_port), cert
        finally:
            http.shutdown()
            thread.join()
            http.server_close()
