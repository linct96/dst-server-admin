log(){
  echo -e "\033[0;34m【信息】\033[0m$1"
}
success(){
  echo -e "\033[0;32m【成功】\033[0m$1"
}

steam_cmd_path="${HOME}/steamcmd"

install_steamCMD() {
  log "开始安装 steamCMD"
  mkdir -p ${steam_cmd_path}
  cd ${steam_cmd_path}
  donwload_steamCMD_url="https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz"

  if [ "${OS}" = "macos" ]; then
    donwload_steamCMD_url="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz"
  fi

  curl -sqkL ${donwload_steamCMD_url} | tar zxf - -C ${steam_cmd_path}
  success "steamCMD 安装成功"
}


main() {
  get_OS
  log "当前系统：${OS}"

  if [ "${OS}" != "macos" ]; then
    # install_lib
    log "当前系统：${OS}"
  fi

  if [ "${OS}" = "macos" ]; then
    steam_exec="steamcmd.sh"
  fi

  if [ -f "${steam_cmd_path}/${steam_exec}" ]; then
    log "steamCMD 已安装"
  else
    log "steamCMD 未安装"
    install_steamCMD
  fi
}

main