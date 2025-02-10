@echo off
echo 正在下载文件...

powershell -Command "Invoke-WebRequest -Uri 'https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip' -OutFile '%0\steamcmd.zip'"
echo 下载完成！