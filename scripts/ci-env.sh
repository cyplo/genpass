#!/usr/bin/env bash
set -e
set -o pipefail

store_host="bolty.raptor-carp.ts.net"
store_address="ssh://nix-ssh@$store_host"

mkdir -p ~/.config/nix
echo "sandbox = false" >> ~/.config/nix/nix.conf
nix shell nixpkgs#tailscale -c tailscale version
nix shell nixpkgs#tailscale -c tailscaled --tun=userspace-networking --socks5-server=localhost:1055 --outbound-http-proxy-listen=localhost:1055 --state="mem:" &
sleep 1
nix shell nixpkgs#tailscale -c tailscale up -authkey $TAILSCALE_KEY
mkdir -p ~/.ssh/
echo "$SSH_PRIVATE_KEY" > ~/.ssh/id_ed25519
echo "$SSH_PUBLIC_KEY" > ~/.ssh/id_ed25519.pub
chmod 0400 ~/.ssh/id_ed25519
ssh-keyscan "$store_host" >> ~/.ssh/known_hosts
echo "substituters = $store_address https://cache.nixos.org/" >> ~/.config/nix/nix.conf
echo "trusted-public-keys = cyplodev-store-key:a/+PEufePs7giWqYyRqy+TgUKLMbY+RQuJQu2aUjdl8= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY=" >> ~/.config/nix/nix.conf
echo "$STORE_SECRET_KEY" > ~/.store-secret-key
