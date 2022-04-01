RunningInDocker() {
    if grep -q docker < /proc/1/cgroup; then
        return true
    else
        return false
    fi
}

Setup() {
    local add_command
    local update_command

    if ! RunningInDocker; then
        echo "Not running in Docker, skipping..."
    fi

    mkdir -p "$HOME"/.config/nix
    echo "sandbox = false" >> "$HOME"/.config/nix/nix.conf
    echo "experimental-features = nix-command flakes" >> "$HOME"/.config/nix/nix.conf

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
