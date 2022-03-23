setup() {
    load 'libs/bats-support/load'
    load 'libs/bats-assert/load'
    source ./src/scripts/install-nix.sh
}

@test '1: Runs on Linux' {
    export BATS_TEST=true
    export BATS_TEST_OS=Linux
    export BATS_TAG_NAME=binary-dev
    run InstallNix
    assert_success
    assert_output "Would run install-nix on Linux"
}

@test '2: Runs on macOS' {
    export BATS_TEST=true
    export BATS_TEST_OS=Darwin
    export BATS_TAG_NAME=binary-dev
    run InstallNix
    assert_success
    assert_output "Would run install-nix on Darwin"
}

@test '3: Errors if OS is not recognized' {
    export BATS_TEST=true
    export BATS_TEST_OS=Unknown
    run InstallNix
    assert_failure
    assert_output "Unsupported operating system"
}