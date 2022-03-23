setup() {
    source ./src/scripts/install-nix.sh
}

@test '1: Runs on Linux' {
    export BATS_TEST=true
    export BATS_TEST_OS=Linux
    export BATS_TAG_NAME=binary-dev
    result=$(InstallNix)
    [ "$result" == "Would run install-nix on Linux" ]
}

@test '2: Runs on macOS' {
    export BATS_TEST=true
    export BATS_TEST_OS=Darwin
    export BATS_TAG_NAME=binary-dev
    result=$(InstallNix)
    [ "$result" == "Would run install-nix on Darwin" ]
}

@test '3: Errors if OS is not recognized' {
    export BATS_TEST=true
    export BATS_TEST_OS=Unknown
    result=$(InstallNix)
    [ "$result" == "Unsupported operating system" ]
}