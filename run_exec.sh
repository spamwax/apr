#!/usr/bin/env bash

alfred_workflow_version=0.11.1 \
alfred_workflow_data=$HOME/tmp/apr \
alfred_workflow_cache=$HOME/tmp/apr \
alfred_workflow_uid=hamid63 \
alfred_workflow_name="Rusty Pin" \
alfred_workflow_bundleid=cc.hamid.alfred-pinboard-rs \
alfred_version=3.6 target/debug/alfred-pinboard-rs "$@"
