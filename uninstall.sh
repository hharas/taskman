#!/bin/bash

set -e

echo "Uninstalling taskman..."
rm /usr/local/bin/taskman

if [ $? -ne 0 ]; then
  echo "Failed to uninstall taskman from /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting..."
