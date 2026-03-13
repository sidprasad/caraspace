#!/usr/bin/env sh
set -eu

example="${1:-rbt}"
if [ "$#" -gt 0 ]; then
    shift
fi

binary="/app/examples/${example}"
if [ ! -x "${binary}" ]; then
    echo "Unknown example '${example}'." >&2
    echo "Available examples:" >&2
    ls -1 /app/examples >&2
    exit 1
fi

echo "Running example '${example}'..."
"${binary}" "$@"

if [ ! -f /tmp/rust_viz_data.html ]; then
    echo "Expected visualization file was not generated at /tmp/rust_viz_data.html" >&2
    exit 1
fi

echo "Starting visualization server..."
exec /usr/local/bin/viz-server /tmp/rust_viz_data.html
