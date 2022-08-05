#!/usr/bin/env python3
from http.server import HTTPServer, SimpleHTTPRequestHandler
import os
import webbrowser
import typer

PORT = 8008
URL = f'http://localhost:{PORT}'


def serve():
    """
    A quick server to preview a built site with translations.

    For development, prefer the command live (or just mkdocs serve).

    This is here only to preview a site with translations already built.

    Make sure you run the build-all command first.
    """
    typer.echo("Warning: this is a very simple server.")
    typer.echo("For development, use the command live instead.")
    typer.echo(
        "This is here only to preview a site with translations already built.")
    typer.echo("Make sure you run the build-all command first.")

    # Set Path to dir
    os.chdir("./dist")

    server = HTTPServer(("", PORT), SimpleHTTPRequestHandler)
    typer.echo(
        f"Serving at: {server.server_address[0]}:{server.server_address[1]}")
    webbrowser.open(URL, new=0)

    server.serve_forever()


serve()
