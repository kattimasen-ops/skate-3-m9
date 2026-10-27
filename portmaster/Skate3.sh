#!/bin/bash
# Skate 3 - PortMaster Launcher fuer RK3326 / M9 Pro / ArkOS4Clone
set -euo pipefail

XDG_DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}
SCRIPT_DIR="$(cd "$(dirname "$0")" 2>/dev/null && pwd)"
GAMEDIR="$SCRIPT_DIR/skate3"

if [ ! -x "$GAMEDIR/skate3" ]; then
    echo "FEHLER: $GAMEDIR/skate3 fehlt." >&2
    exit 1
fi

controlfolder=""
for candidate in \
    "/opt/system/Tools/PortMaster" \
    "/opt/tools/PortMaster" \
    "$XDG_DATA_HOME/PortMaster" \
    "/roms/ports/PortMaster" \
    "/roms/tools/PortMaster"; do
    if [ -d "$candidate" ]; then controlfolder="$candidate"; break; fi
done

if [ -n "$controlfolder" ] && [ -f "$controlfolder/control.txt" ]; then
    source "$controlfolder/control.txt" 2>/dev/null || true
    if [ -f "${controlfolder}/mod_${CFW_NAME}.txt" ]; then
        source "${controlfolder}/mod_${CFW_NAME}.txt" 2>/dev/null || true
    fi
fi

export SDL_VIDEODRIVER="${SDL_VIDEODRIVER:-kmsdrm}"
unset SDL_AUDIODRIVER AUDIODEV

if [ -n "${sdl_controllerconfig:-}" ]; then
    export SDL_GAMECONTROLLERCONFIG="$sdl_controllerconfig"
fi
if [ -n "${SDL_GAMECONTROLLERCONFIG:-}" ]; then
    export SDL_GAMECONTROLLERCONFIG_FILE="/tmp/skate3-gamecontrollerdb.txt"
    printf '%s\n' "$SDL_GAMECONTROLLERCONFIG" > "$SDL_GAMECONTROLLERCONFIG_FILE" 2>/dev/null || true
fi

# Mali-Symlinks (WOTS2-Erkenntnis)
SYSTEM_MALI=""
for candidate in \
    /usr/lib/aarch64-linux-gnu/libmali-bifrost-g31-rxp0-gbm.so \
    /usr/local/lib/aarch64-linux-gnu/libmali-bifrost-g31-rxp0-gbm.so \
    /usr/lib/aarch64-linux-gnu/libmali.so.1 \
    /usr/lib/aarch64-linux-gnu/libmali.so \
    /usr/lib/aarch64-linux-gnu/libEGL.so.1; do
    if [ -f "$candidate" ]; then SYSTEM_MALI="$candidate"; break; fi
done
if [ -n "$SYSTEM_MALI" ]; then
    rm -rf /tmp/skate3-mali
    mkdir -p /tmp/skate3-mali
    for name in libmali.so.0 libmali.so.1 libmali.so libEGL.so.1 libEGL.so libGLESv2.so.2 libGLESv2.so; do
        ln -sf "$SYSTEM_MALI" "/tmp/skate3-mali/$name"
    done
    export LD_LIBRARY_PATH="/tmp/skate3-mali:$GAMEDIR/libs.aarch64:$GAMEDIR:/usr/lib/aarch64-linux-gnu:${LD_LIBRARY_PATH:-}"
else
    export LD_LIBRARY_PATH="$GAMEDIR/libs.aarch64:$GAMEDIR:/usr/lib/aarch64-linux-gnu:${LD_LIBRARY_PATH:-}"
fi

# Runtime-Tuning (WOTS2-Erkenntnisse)
export MALLOC_TRIM_THRESHOLD_=131072
export MALLOC_MMAP_THRESHOLD_=131072
export SDL_KMSDRM_DOUBLE_BUFFER=1
export SDL_AUDIO_SAMPLES="${SDL_AUDIO_SAMPLES:-2048}"

# Skate-3 Performance-Defaults (Vita + WOTS2-Hebel)
export SKATE3_NO_VSYNC=1
export SKATE3_NO_SHADOW=1
export SKATE3_NO_POST=0
export SKATE3_RENDER_WIDTH=448
export SKATE3_RENDER_HEIGHT=336
export SKATE3_NPC_DRAW_DIST=10
export SKATE3_TRAFFIC_DENSITY=0.25
export SKATE3_PED_DENSITY=0.25

# CPU-Governor (WOTS2-Erkenntnis)
cpu_gov=""
for c in /sys/devices/system/cpu/cpufreq/policy0/scaling_governor \
         /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor; do
    if [ -w "$c" ]; then cpu_gov="$c"; break; fi
done
if [ -n "$cpu_gov" ]; then echo performance > "$cpu_gov" 2>/dev/null || true; fi

# gptokeyb Killswitch
GPTOKEYB=""
for c in "$controlfolder/gptokeyb" /opt/system/Tools/PortMaster/gptokeyb \
         /usr/local/bin/gptokeyb /usr/bin/gptokeyb; do
    if [ -x "$c" ]; then GPTOKEYB="$c"; break; fi
done
GPTOKEYB_PID=""
if [ -n "$GPTOKEYB" ]; then
    "$GPTOKEYB" "$GAMEDIR/skate3" -k skate3 &
    GPTOKEYB_PID=$!
fi

cd "$GAMEDIR"
STATUS=0
./skate3 "$@" || STATUS=$?

if [ -n "$GPTOKEYB_PID" ]; then kill -TERM "$GPTOKEYB_PID" 2>/dev/null || true; fi
if [ -n "$cpu_gov" ]; then echo ondemand > "$cpu_gov" 2>/dev/null || true; fi

exit "$STATUS"
