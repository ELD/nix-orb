InstallNix() {
    local os=$(uname)

    if [[ os = "Darwin" ]]; then
        ./darwin/install-nix < /dev/null
    elif [[ os = "Linux" ]]; then
        ./linux/install-nix < /dev/null
    else
        echo "Unsupported operating system"
        exit 1
    fi
}

ORB_TEST_ENV="bats-core"
if [ "${0#*$ORB_TEST_ENV}" == "$0" ]; then
    InstallNix
fi
