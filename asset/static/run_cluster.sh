#!/bin/sh

# 检查参数数量是否正确
if [ "$#" -ne 3 ]; then
  echo "Usage: $0 <bin_path> <cluster_name> <world_name>"
  exit 1
fi

# 获取参数
bin_path=$1
cluster_name=$2
world_name=$3

OS=""

# HomePath="${HOME}"
# bin_path="$HomePath/Library/Application Support/Steam/steamapps/common/Don't Starve Together Dedicated Server/dontstarve_dedicated_server_nullrenderer.app/Contents/MacOS"
# cluster_name="cluster_1"
# world_name="Master"

log(){
  echo -e "\033[0;34m【信息】\033[0m$1"
}

get_OS() {
  if [ "$(uname -s)" = "Darwin" ]; then
    OS="macos"
  elif [ -f /etc/redhat-release ]; then
    OS="centos"
  elif cat /proc/version | grep -q -E -i "centos|red hat|redhat"; then
    OS="centos"
  elif cat /proc/version | grep -q -E -i "debian"; then
    OS="debian"
  elif cat /proc/version | grep -q -E -i "ubuntu"; then
    OS="ubuntu"
  else
    OS=""
  fi
}

run_cluster() {
  # if [ "$OS" = "macos" ]; then
  #   cd "$HomePath/Library/Application Support/Steam/steamapps/common/Don't Starve Together Dedicated Server/dontstarve_dedicated_server_nullrenderer.app/Contents/MacOS"
  # else
  #   cd "$HomePath/Steam/steamapps/common/Don't Starve Together Dedicated Server/bin"
  # fi
  cd "$bin_path"
  run_shared=(screen -d -m -S "$cluster_name-$world_name")
  run_shared+=(./dontstarve_dedicated_server_nullrenderer)
  run_shared+=(-console_enabled)
  run_shared+=(-region sing)
  run_shared+=(-monitor_parent_process $)
  # run_shared+=(-ugc_directory "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/ugc_mods")
  run_shared+=(-cluster "$cluster_name")
  run_shared+=(-shard "$world_name")
  "${run_shared[@]}"
  log "$cluster_name-$world_name 启动成功"
}

get_OS
run_cluster