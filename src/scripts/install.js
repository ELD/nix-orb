const fs = require('fs');
const { spawn } = require('child_process');

const nixInstallerUrl = 'https://install.determinate.systems/nix';

// Check for existing install
if (fs.existsSync('/nix/receipt.json') && fs.existsSync('/nix/nix-installer')) {
  if (process.env.REINSTALL) {
    spawn('/nix/nix-installer uninstall --no-confirm', { stdio: 'inherit', shell: true });
    process.exit(1);
  } else {
    console.log('Nix was already installed, using existing install');
    spawn(`echo 'export path=/nix/var/nix/profiles/default/bin:"$PATH"' >> "$BASH_ENV"`, { stdio: 'inherit', shell: true })
    process.exit(0);
  }
}

// Set environment variables
if (process.env.CHANNELS && process.env.CHANNELS !== '') {
  process.env.NIX_INSTALLER_CHANNELS = process.env.CHANNELS;
  console.log(`Set NIX_INSTALLER_CHANNELS=${process.env.NIX_INSTALLER_CHANNELS}`);
}

if (process.env.MODIFY_PROFILE && !!Number(process.env.MODIFY_PROFILE)) {
  process.env.NIX_INSTALLER_MODIFY_PROFILE = process.env.MODIFY_PROFILE;
  console.log(`Set NIX_INSTALLER_MODIFY_PROFILE=${process.env.NIX_INSTALLER_MODIFY_PROFILE}`);
}

if (process.env.NIX_BUILD_USER_COUNT && Number(process.env.NIX_BUILD_USER_COUNT) !== -1) {
  process.env.NIX_INSTALLER_NIX_BUILD_USER_COUNT = process.env.NIX_BUILD_USER_COUNT;
  console.log(`Set NIX_INSTALLER_NIX_BUILD_USER_COUNT=${process.env.NIX_INSTALLER_NIX_BUILD_USER_COUNT}`);
}

if (process.env.NIX_BUILD_GROUP_NAME && process.env.NIX_BUILD_GROUP_NAME !== '') {
  process.env.NIX_INSTALLER_NIX_BUILD_GROUP_NAME = process.env.NIX_BUILD_GROUP_NAME;
  console.log(`Set NIX_INSTALLER_NIX_BUILD_GROUP_NAME=${process.env.NIX_INSTALLER_NIX_BUILD_GROUP_NAME}`);
}

if (process.env.NIX_BUILD_GROUP_ID && Number(process.env.NIX_BUILD_GROUP_ID) !== -1) {
  process.env.NIX_INSTALLER_NIX_BUILD_GROUP_ID = process.env.NIX_BUILD_GROUP_ID;
  console.log(`Set NIX_INSTALLER_NIX_BUILD_GROUP_ID=${process.env.NIX_INSTALLER_NIX_BUILD_GROUP_ID}`);
}

if (process.env.NIX_BUILD_USER_PREFIX && process.env.NIX_BUILD_USER_PREFIX !== '') {
  process.env.NIX_INSTALLER_NIX_BUILD_USER_PREFIX = process.env.NIX_BUILD_USER_PREFIX;
  console.log(`Set NIX_INSTALLER_NIX_BUILD_USER_PREFIX=${process.env.NIX_INSTALLER_NIX_BUILD_USER_PREFIX}`);
}

if (process.env.NIX_BUILD_USER_BASE && Number(process.env.NIX_BUILD_USER_BASE) !== -1) {
  process.env.NIX_INSTALLER_NIX_BUILD_USER_BASE = process.env.NIX_BUILD_USER_BASE;
  console.log(`Set NIX_INSTALLER_NIX_BUILD_USER_BASE=${process.env.NIX_INSTALLER_NIX_BUILD_USER_BASE}`);
}

if (process.env.NIX_PACKAGE_URL && process.env.NIX_PACKAGE_URL !== '') {
  process.env.NIX_INSTALLER_NIX_PACKAGE_URL = process.env.NIX_PACKAGE_URL;
  console.log(`Set NIX_INSTALLER_NIX_PACKAGE_URL=${process.env.NIX_INSTALLER_NIX_PACKAGE_URL}`);
}

if (process.env.NIX_EXTRA_CONF && process.env.NIX_EXTRA_CONF !== '') {
  process.env.NIX_INSTALLER_EXTRA_CONF = `${process.env.NIX_EXTRA_CONF}\ntrusted-users = root ${process.env.USER}`;
  console.log(`Set NIX_INSTALLER_EXTRA_CONF=${process.env.NIX_EXTRA_CONF}`);
}

if (process.env.MAC_ENCRYPT && !!Number(process.env.MAC_ENCRYPT)) {
  process.env.NIX_INSTALLER_ENCRYPT = process.env.MAC_ENCRYPT;
  console.log(`Set NIX_INSTALLER_ENCRYPT=${process.env.NIX_INSTALLER_ENCRYPT}`);
}

if (process.env.MAC_CASE_SENSITIVE && !!Number(process.env.MAC_CASE_SENSITIVE)) {
  process.env.NIX_INSTALLER_CASE_SENSITIVE = process.env.MAC_CASE_SENSITIVE;
  console.log(`Set NIX_INSTALLER_CASE_SENSITIVE=${process.env.NIX_INSTALLER_CASE_SENSITIVE}`);
}

if (process.env.MAC_VOLUME_LABEL && process.env.MAC_VOLUME_LABEL !== '') {
  process.env.NIX_INSTALLER_VOLUME_LABEL = process.env.MAC_VOLUME_LABEL;
  console.log(`Set NIX_INSTALLER_VOLUME_LABEL=${process.env.NIX_INSTALLER_VOLUME_LABEL}`);
}

if (process.env.MAC_ROOT_DISK && process.env.MAC_ROOT_DISK !== '') {
  process.env.NIX_INSTALLER_ROOT_DISK = process.env.MAC_ROOT_DISK;
  console.log(`Set NIX_INSTALLER_ROOT_DISK=${process.env.NIX_INSTALLER_ROOT_DISK}`);
}

if (process.env.LOGGER && process.env.LOGGER !== '') {
  process.env.NIX_INSTALLER_LOGGER = process.env.LOGGER;
  console.log(`Set NIX_INSTALLER_LOGGER=${process.env.NIX_INSTALLER_LOGGER}`);
}

if (process.env.INIT && process.env.INIT !== '') {
  process.env.NIX_INSTALLER_INIT = process.env.INIT;
  console.log(`Set NIX_INSTALLER_INIT=${process.env.NIX_INSTALLER_INIT}`);
}

if (process.env.START_DAEMON && process.env.START_DAEMON !== '') {
  process.env.NIX_INSTALLER_START_DAEMON = process.env.START_DAEMON;
  console.log(`Set NIX_INSTALLER_START_DAEMON=${process.env.NIX_INSTALLER_START_DAEMON}`);
}

process.env.NIX_INSTALLER_NO_CONFIRM = true;
console.log(`Set NIX_INSTALLER_NO_CONFIRM=${process.env.NIX_INSTALLER_NO_CONFIRM}`);

// Install Nix
spawn(`curl --retry 20 -L ${nixInstallerUrl} | sh -s -- install ${process.env.PLANNER}`, { stdio: 'inherit', shell: true });
spawn(`echo 'export PATH=/nix/var/nix/profiles/per-user/${process.env.USER}/profile/bin:/nix/var/nix/profiles/default/bin:"$PATH"' >> "$BASH_ENV"`, { stdio: 'inherit', shell: true });
