#!/bin/bash
# Tekipaki OS Reboot Countdown

for i in {10..1}
do
    # Use kdialog to show a passive popup that updates
    kdialog --icon "system-reboot" --passivepopup "インストールが完了しました。 $i 秒後に自動的に再起動します... / Installation complete. Auto-rebooting in $i seconds..." 1
    sleep 1
done

reboot
