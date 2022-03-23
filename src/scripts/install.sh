CheckPreconditions() {
    if ! command -v curl >& /dev/null; then
        echo "curl is required to use this command"
        exit 1
    fi
}

InstallNix() {
    local os
    local download_url
    local target_triple
    local tag_name

    if ! CheckPreconditions; then
        echo "Preconditions checking failed... exiting"
        exit 1
    fi

    os=${BATS_TEST_OS:-$(uname)}
    if [[ $os = "Darwin" ]]; then
        target_triple="x86_64-apple-darwin"
    elif [[ $os = "Linux" ]]; then
        target_triple="x86_64-linux-unknown-musl"
    else
        echo "Unsupported operating system"
        exit 1
    fi

    tag_name="${BATS_TAG_NAME:-${TAG_NAME}}"
    download_url="https://github.com/ELD/nix-orb/releases/download/${tag_name}/install-nix-${target_triple}"
    mkdir -p "${HOME}/.bin/nix-orb"

    if [[ "${BATS_TEST}" == "true" ]]; then
        echo "Would run install-nix on ${os}"
        exit 0
    fi

    local curl_retries
    curl_retries=5
    while ! curl -o "${HOME}/.bin/nix-orb/install-nix" -L --fail "${download_url}" >& /dev/null; do
        sleep 1
        ((curl_retries--))
        if [[ $curl_retries -le 0 ]]; then
            echo "unable to download binary" >& 2
            exit 1
        fi
    done

    chmod +x "$HOME"/.bin/nix-orb/install-nix
    # Close stdin so the installer runs non-interactively
    "$HOME"/.bin/nix-orb/install-nix < /dev/null

    # shellcheck source=/dev/null
    . "$BASH_ENV"
}

ORB_TEST_ENV="bats-core"
if [ "${0#*"$ORB_TEST_ENV"}" == "$0" ]; then
    InstallNix
fi
