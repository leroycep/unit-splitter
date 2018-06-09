#!/usr/bin/env bash

echo "Generating changelog.html"
comrak CHANGELOG.md > static/changelog.html
