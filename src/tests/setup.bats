setup() {
    load 'libs/bats-support/load'
    load 'libs/bats-assert/load'
    source './src/scripts/setup.sh'
}

@test '1: Creates and writes nix config' {
    export BATS_TEST=true
    run Setup
    assert_success
    assert [ -e "$HOME"/config/nix.conf ]
}

@test '2: Creates the nix-channel add command' {
    export BATS_TEST=true
    export NIX_CHANNEL="nixpkgs-unstable"
    run Setup
    assert_success
    assert_output --partial "nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs"
    assert_output --partial "nix-channel --update"
}
