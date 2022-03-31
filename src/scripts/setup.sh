Setup() {
    local add_command
    local update_command
    mkdir -p "$HOME"/.config/nix
    echo "sandbox = false" >> "$HOME"/.config/nix

    add_command="nix-channel --add https://nixos.org/channels/$NIX_CHANNEL nixpkgs"
    update_command="nix-channel --update"

    if [[ "$BATS_TEST" == "true" ]]; then
        echo "$add_command"
        echo "$update_command"
        exit 0
    fi

    eval "$add_command"
    eval "$update_command"
}

ORB_TEST_ENV="bats-core"
if [ "${0#*"$ORB_TEST_ENV"}" == "$0" ]; then
    Setup
fi
