# Overview

This is a Proof of oncept changing how we deliver charm hooks to deploy an application. I have reproduced the behaviour of the ubuntu charm in this demo, using [Rust] as the charm language. The interesting thing about this approach is that it can allow us to handle architecture specific charms + compiled languages, which has historically been a difficult situation. In addition, it can be used to reduce charm setup time, as we can ship a python environment, complete with all packages, as a charm environment.

# Usage

`make deploy` is a target that should handle the snapcraft step, as well as deploying this charm with the built resource correctly defined.

Alternately, `snapcraft clean` followed by `juju deploy . --resource charm-snap=ubuntu-charm_0.1_amd64.snap` will build the snap and then deploy the charm.