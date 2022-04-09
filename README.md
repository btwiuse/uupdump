uupdump
=======

Run this tool from linux terminal to interactively retrieve uuid of recent Win11 iso images.  

Then pass retrieved uuid to the following shell function to start downloading iso in current directory

```
dump(){
  uuid="$1"
  if [[ -z "$uuid" ]]; then
    echo 'wrong num of args'
    return
  fi
  curl -sL "https://uupdump.net/get.php?id=${uuid}&pack=zh-cn&edition=PROFESSIONAL&autodl=2" | docker run -i -w $PWD -v $PWD:$PWD busybox unzip -o -
  # docker run -dit -w $PWD -v $PWD:$PWD winiso/uupdump uup_download_linux.sh
  docker run -it -v /etc/:/etc/ --net=host -w $PWD -v $PWD:$PWD winiso/uupdump uup_download_linux.sh
}
```

See Also

- https://github.com/AveYo/MediaCreationTool.bat
