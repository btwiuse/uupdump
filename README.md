uupdump
=======

Interactively retrieve uuid of recent Win11 iso images from uupdump.net.

```
$ uupdump
  x64      Update Stack Package - (Version 922.402.211.0) amd64                             bc6d9fc7-24ac-4a62-825a-09f203265a35         2022-04-07 17:00:56 UTC
  arm64    Update Stack Package - (Version 922.402.211.0) arm64                             f5a15d63-b83d-40e6-ad39-85812ef1cc00         2022-04-07 17:00:56 UTC
❯ x64      Windows 11 Insider Preview 22593.1 (ni_release) amd64                            1508fcad-cc97-480b-be61-4848858d3e35         2022-04-06 17:00:07 UTC
  arm64    Windows 11 Insider Preview 22593.1 (ni_release) arm64                            bf7c5e97-71d3-4f43-bd7a-1987320576c5         2022-04-06 17:00:06 UTC
  x86      Update for Windows 10 Version 21H2 - KB4023057 (19041.1675) x86                  3b35a17c-7c98-46e0-a44f-efe2b8d0a0b8         2022-04-05 00:02:07 UTC
  x64      Update for Windows 11 - KB4023057 (22000.647) amd64                              2b58ec03-8020-4610-b59d-a56c74c66aa2         2022-04-04 21:03:11 UTC
  x64      Update for Windows 10 Version 21H2 - KB4023057 (19041.1675) amd64                a329b681-ce8c-431d-99f7-052e2901adcb         2022-04-04 21:01:28 UTC
  x64      Update for Windows 10 Version 20H2 - KB4023057 (19041.1675) amd64                7e2daed7-06e5-493e-b1ec-bb7791cc3139         2022-04-04 21:00:46 UTC
  x86      Update for Windows 10 Version 20H2 - KB4023057 (19041.1675) x86                  8902b41a-9a2b-47b4-b5eb-0fbd3d0dc6e9         2022-04-04 21:00:46 UTC
  x64      Update for Windows 10 Version 21H1 - KB4023057 (19041.1675) amd64                0d20bdba-871f-439d-83f8-b6b12a704fc6         2022-04-04 21:00:22 UTC
  x86      Update for Windows 10 Version 21H1 - KB4023057 (19041.1675) x86                  f533a19e-fd7f-4210-8fb8-17df699781d2         2022-04-04 21:00:22 UTC
  arm64    Cumulative Update Preview for Windows 10 Version 21H2 (19044.1620) arm64         b58d9116-62ae-4828-adfc-b0abaf7215af         2022-04-04 14:11:54 UTC
  x64      Cumulative Update Preview for Windows 10 Version 20H2 (19042.1620) amd64         0f5eeb54-5280-4bba-9b03-87b09111fc55         2022-04-02 10:42:13 UTC
  arm64    Cumulative Update Preview for Windows 10 Version 20H2 (19042.1620) arm64         97c07a09-7223-41f2-b048-d4675020a0ba         2022-04-02 10:42:13 UTC
  x86      Cumulative Update Preview for Windows 10 Version 20H2 (19042.1620) x86           ac1822ec-0035-4a68-ad44-12ec88fde18f         2022-04-02 10:42:13 UTC
```

It's part of the docker image btwiuse/uupdump, which allows you to download win11 iso images on any linux that runs docker (including WSL2).

```
$ docker run -it -v /uupdump:/uupdump -w /uupdump btwiuse/uupdump
Archive:  -
  inflating: uup_download_windows.cmd
  inflating: uup_download_linux.sh                                                                                                                                                                                 inflating: uup_download_macos.sh
  inflating: ConvertConfig.ini                                                                                                                                                                                     inflating: files/convert_config_linux
  inflating: files/convert_config_macos                                                                                                                                                                            inflating: files/aria2c.exe
  inflating: files/convert.sh                                                                                                                                                                                      inflating: files/convert_ve_plugin                                                                                                                                                                               inflating: files/7zr.exe                                                                                                                                                                                         inflating: files/uup-converter-wimlib.7z
Retrieving aria2 script...

04/09 15:55:54 [NOTICE] Downloading 1 item(s)
[#2dd254 0B/0B CN:1 DL:0B]                                                                                                                                                                                       04/09 15:55:57 [NOTICE] Download complete: /uupdump/aria2_script.5623.txt

Download Results:
gid   |stat|avg speed  |path/URI
======+====+===========+=======================================================
2dd254|OK  |   8.4KiB/s|/uupdump/aria2_script.5623.txt

Status Legend:
(OK):download completed.

Attempting to download files...
```

After finished, your iso file is located under /uupdump, for example `/uupdump/22593.1_PROFESSIONAL_X64_ZH-CN.ISO`

See Also

- https://github.com/AveYo/MediaCreationTool.bat
