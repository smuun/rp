#!/bin/bash

echo "Build rp and copy binary to ~/.local/bin/rp.  Proceed?"
select yn in "Yes" "No"; do
  case $yn in
    Yes) break;;
    No ) exit;;
  esac
done
set -e
mkdir -p ~/.local/bin
cargo build --release 
cp ./target/release/rp ~/.local/bin/rp
chmod +x ~/.local/bin/rp 
echo Done.  Make sure ~/.local/bin is in your \$PATH.
