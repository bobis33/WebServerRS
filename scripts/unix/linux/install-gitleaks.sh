#!/bin/bash
set -euo pipefail

version=8.30.0
platform=linux

wget https://github.com/gitleaks/gitleaks/releases/download/v${version}/gitleaks_${version}_${platform}_x64.tar.gz
tar -xf gitleaks_${version}_${platform}_x64.tar.gz
sudo mv gitleaks /usr/local/bin/
