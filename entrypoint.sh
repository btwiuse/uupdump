#!/usr/bin/env bash

dump(){
  uuid="$1"
  if ! [[ -n "$uuid" ]]; then
    uuid="$(uupdump)"
  fi
  if ! [[ -n "$uuid" ]]; then
    echo 'wrong num of args'
    return
  fi
  echo curl -sL "https://uupdump.net/get.php?id=${uuid}&pack=zh-cn&edition=PROFESSIONAL&autodl=2"
  curl -sL "https://uupdump.net/get.php?id=${uuid}&pack=zh-cn&edition=PROFESSIONAL&autodl=2" > zip
  cat zip | busybox unzip -o -
  bash ./uup_download_linux.sh
}

dump "$@"
