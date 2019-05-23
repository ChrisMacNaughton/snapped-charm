ubuntu-charm_0.1_amd64.snap: snap/snapcraft.yaml src
	snapcraft build

deploy: ubuntu-charm_0.1_amd64.snap
	juju deploy . --resource charm-snap=ubuntu-charm_0.1_amd64.snap