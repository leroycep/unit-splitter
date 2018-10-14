#!/usr/bin/env bash

echo "Generating changelog.html"

OUT=static/changelog.html

echo "" > $OUT
echo '
<!doctype html>
<html lang="en">
    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta charset="utf-8">
        <title>Unit Splitter - Changelog</title>
        <link rel="stylesheet" href="reset.css">
        <link rel="stylesheet" href="styles.css">
    </head>
    <body class="container">
        <header>Unit Splitter - Changelog</header>
        <nav></nav>
        <main>
' >> $OUT

comrak CHANGELOG.md >> $OUT

echo '
        </main>
        <aside></aside>
        <footer>LeRoyce Pearson &lt;leroycepearson@geemili.xyz&gt;</footer>
    </body>
</html>
' >> $OUT
