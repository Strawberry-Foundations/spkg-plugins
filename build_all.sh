#!/bin/bash
# BLACK="\e[30m"
# RED="\e[31m"
# GREEN="\e[32m"
# YELLOW="\e[33m"
CYAN="\e[36m"

YELLOW_BACK="\e[43m"
GREEN_BACK="\e[42m"
# RED_BACK="\e[41m"

BOLD="\e[1m"
RESET="\e[0m"

BASE_DIRECTORY=$(pwd)
RELEASE_DIRECTORY="${BASE_DIRECTORY}/releases"

mkdir -p "${RELEASE_DIRECTORY}"

echo -e "${YELLOW_BACK}${BOLD}  INFO  ${RESET}  Starting plugin build process"
echo -e "          * ------------------------- *"

for directory in './'*; do
    if [[ -d "${directory}" ]]; then
        if [ ! -f "${directory}/Cargo.toml" ]; then
            continue
        fi
        echo -e ""
        plugin="${directory#./}"
        echo -e "${YELLOW_BACK}${BOLD}  INFO  ${RESET}  Building plugin '${CYAN}${plugin}${RESET}'"
        cd "${directory}" || return
        cargo build --release

        SOURCE_DIR="${BASE_DIRECTORY}/${plugin}/target/release/"
        FILE="${SOURCE_DIR}lib${plugin}.so"

        if [ -e "$FILE" ]; then
            cp "${FILE}" "${RELEASE_DIRECTORY}"
            echo -e "${GREEN_BACK}${BOLD}   OK   ${RESET}  Plugin build available at '${CYAN}${RELEASE_DIRECTORY}/$(basename "${FILE}")${RESET}'"
        fi
        cd "${BASE_DIRECTORY}" || return
        sleep 0.1   
    fi
done
