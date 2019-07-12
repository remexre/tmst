#!/bin/bash

set -eu

# DMENU="dmenu"
DMENU="rofi -dmenu"
NOTIFY_SEND="notify-send"
TMST="tmst"

if [[ $($TMST current -f "1" --else "0") = 1 ]]; then
	$NOTIFY_SEND "$($TMST current -f "Clocked %h hours to %p.")"
	$TMST out
else
	project=$($TMST list-projects | $DMENU)
	$TMST in $project
fi

set +e
pkill -SIGRTMIN+10 i3blocks
