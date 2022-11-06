#!/usr/bin/env bash

dl(){
  curl "https://uupdump.net/get.php?id=${uuid}&pack=en-us&edition=professional" \
    -H 'authority: uupdump.net' \
    -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9' \
    -H 'accept-language: en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7' \
    -H 'cache-control: max-age=0' \
    -H 'content-type: application/x-www-form-urlencoded' \
    -H 'cookie: cf_clearance=4b1bb5337bea7e820d30c9683556a63cc8e86a57-1667749095-0-150' \
    -H 'origin: https://uupdump.net' \
    -H 'referer: https://uupdump.net/download.php?id=d939f013-aa0c-4d15-8794-bf631a4a969b&pack=en-us&edition=professional' \
    -H 'sec-ch-ua: "Microsoft Edge";v="107", "Chromium";v="107", "Not=A?Brand";v="24"' \
    -H 'sec-ch-ua-mobile: ?0' \
    -H 'sec-ch-ua-platform: "Windows"' \
    -H 'sec-fetch-dest: document' \
    -H 'sec-fetch-mode: navigate' \
    -H 'sec-fetch-site: same-origin' \
    -H 'sec-fetch-user: ?1' \
    -H 'upgrade-insecure-requests: 1' \
    -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.35' \
    --data-raw 'autodl=3&updates=1&cleanup=1&netfx=1&esd=1&virtualEditions%5B%5D=ProfessionalWorkstation&virtualEditions%5B%5D=ProfessionalEducation&virtualEditions%5B%5D=Education&virtualEditions%5B%5D=Enterprise&virtualEditions%5B%5D=ServerRdsh&virtualEditions%5B%5D=IoTEnterprise' \
    --compressed
}

dump(){
  uuid="$1"
  if ! [[ -n "$uuid" ]]; then
    echo 'wrong num of args'
    return
  fi
  echo curl -sL "https://uupdump.net/get.php?id=${uuid}&pack=en-us&edition=professional"
  dl ${uuid} > zip
  cat zip | busybox unzip -o -
  bash ./uup_download_linux.sh
}

dump "$@"
