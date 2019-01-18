#!/usr/bin/env bash

make
scp -r target/unit-splitter geemili_utilsgeemili@ssh.phx.nearlyfreespeech.net:/home/public/

