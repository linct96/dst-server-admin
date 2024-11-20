#!/bin/bash
get_OS() {
  if [ "$(uname -s)" = "Darwin" ]; then
    OS="macos"
  elif [ -f /etc/redhat-release ]; then
    OS="centos"
  elif lsb_release -a | grep -q -E -i "centos|red hat|redhat"; then
    OS="centos"
  elif lsb_release -a | grep -q -E -i "debian"; then
    OS="debian"
  elif lsb_release -a | grep -q -E -i "ubuntu"; then
    OS="ubuntu"
  else
    OS=""
  fi
}
get_OS
echo "当前系统：${OS}"