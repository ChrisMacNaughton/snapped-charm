#!/bin/bash

set -u

export PATH="$PATH:/snap/bin"
snap_name=$(config-get 'snap_name')

resource=$(resource-get charm-snap)
echo "Installing snap: ${resource}"
snap install --dangerous --classic ${resource}


if $(command -v ${snap_name}.run-hook); then
    ${snap_name}.run-hook
else
    ${snap_name}.$0
fi