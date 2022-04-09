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
  curl -sL "https://uupdump.net/get.php?id=${uuid}&pack=zh-cn&edition=PROFESSIONAL&autodl=2" | busybox unzip -o -
  bash ./uup_download_linux.sh
}

dump "$@"
