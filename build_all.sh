#!/bin/bash
BLACK="\e[30m"
RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
CYAN="\e[36m"

YELLOW_BACK="\e[43m"
GREEN_BACK="\e[42m"
RED_BACK="\e[41m"

BOLD="\e[1m"
RESET="\e[0m"

BASE_DIRECTORY=$(pwd)
RELEASE_DIRECTORY="${BASE_DIRECTORY}/releases"

mkdir -p "${RELEASE_DIRECTORY}"

for directory in './'*; do
    if [[ -d "${directory}" && ! -L "${directory}" ]]; then
        plugin="${directory#./}"
        echo -e "${YELLOW_BACK}${BOLD}  INFO  ${RESET}  Building plugin '${CYAN}${plugin}${RESET}'"
        cd "${directory}" || return
        cargo build --release

        SOURCE_DIR="${BASE_DIRECTORY}/${plugin}/target/release/"

        for file in "$SOURCE_DIR"/*.so; do 
            if [ -e "$file" ]; then
                cp "${file}" "${RELEASE_DIRECTORY}"
                echo -e "${GREEN_BACK}${BOLD}   OK   ${RESET}  Plugin build available at '${CYAN}${RELEASE_DIRECTORY}/$(basename "$file")${RESET}'"
            fi
        done
    fi
done
