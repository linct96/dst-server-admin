#!/bin/sh

# 检查参数数量是否正确
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <param1> <param2>"
  exit 1
fi

# 获取参数
cluster_name=$1
world_name=$2

OS=""
HomePath="${HOME}"

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

# cd "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/bin"
# run_shared=(./dontstarve_dedicated_server_nullrenderer)
# run_shared+=(-console_enabled)
# run_shared+=(-cluster "ddd")
# run_shared+=(-ugc_directory "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/ugc_mods")
# run_shared+=(-region sing)
# run_shared+=(-monitor_parent_process $)
# run_shared+=(-shard "Forest1")
# "${run_shared[@]}"
run_cluster() {
  if [ "$OS" = "macos" ]; then
    cd "$HomePath/Library/Application Support/Steam/steamapps/common/Don't Starve Together Dedicated Server/dontstarve_dedicated_server_nullrenderer.app/Contents/MacOS"
  else
    cd "$HomePath/Steam/steamapps/common/Don't Starve Together Dedicated Server/bin"
  fi
  run_shared=(./dontstarve_dedicated_server_nullrenderer)
  run_shared+=(-console_enabled)
  run_shared+=(-region sing)
  run_shared+=(-monitor_parent_process $)
  # run_shared+=(-ugc_directory "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/ugc_mods")
  run_shared+=(-cluster "$cluster_name")
  run_shared+=(-shard "$world_name")
  "${run_shared[@]}"
}

main() {
  get_OS
  run_cluster
}

main
