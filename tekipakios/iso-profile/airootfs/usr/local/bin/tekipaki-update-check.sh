#!/bin/bash
# Tekipaki OS Background Update Service
# This script downloads updates in the background and notifies the user.

LOG_FILE="/var/log/tekipaki-update.log"

echo "$(date): Checking for updates..." >> "$LOG_FILE"

# Download updates without installing
# --noconfirm is used for downloading only
# --needed prevents re-downloading
if pacman -Syuw --noconfirm >> "$LOG_FILE" 2>&1; then
    # Check if there are actual updates downloaded
    # pacman -Qu returns 0 if there are updates, 1 if not.
    if pacman -Qu > /dev/null; then
        echo "$(date): Updates downloaded and ready." >> "$LOG_FILE"

        # Notify the user (using notify-send if a session is active)
        # We try to find the active user session
        USER_ID=$(who | awk '{print $1}' | head -n 1)
        if [ -n "$USER_ID" ]; then
            USER_DBUS_ADDR="unix:path=/run/user/$(id -u $USER_ID)/bus"
            sudo -u "$USER_ID" DBUS_SESSION_BUS_ADDRESS="$USER_DBUS_ADDR" \
                notify-send "Tekipaki Update" "システムアップデートの準備ができました。設定またはPamacからインストールしてください。" \
                --icon=system-software-update --urgency=normal
        fi
    else
        echo "$(date): No updates found." >> "$LOG_FILE"
    fi
else
    echo "$(date): Update check/download failed." >> "$LOG_FILE"
fi
