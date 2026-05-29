#!/bin/bash

# Name of the development container
NAME=dev-rust

# List of packages required in the container
PKGS="cargo
    wayland-devel
    g++
    alsa-lib-devel
    libudev-devel
    libxkbcommon"

# Create the container only if it does not already exist
exists=$(toolbox list --containers | awk '$2 == "dev-rust"' | wc -l)
if [[ $exists -eq 0 ]]
then # Create the toolbox container
    toolbox create "${NAME}" --distro fedora --release 44
fi

# Update and initialize the container
toolbox run -c "${NAME}" sudo dnf --assumeyes update
toolbox run -c "${NAME}" sudo dnf --assumeyes install ${PKGS}

