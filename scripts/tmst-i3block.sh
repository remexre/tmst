#!/bin/bash

set -eu

TMST="tmst"

$TMST current -f "%p (%hhr)" --else ""
$TMST current -f "%p (%h)" --else ""
echo "#49BAFF"
