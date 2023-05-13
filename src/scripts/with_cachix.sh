if [[ -n "$CACHE_NAME" ]]; then
  cachix use "$CACHE_NAME"
fi

echo "Executing: cachix watch-exec $CACHIX_USER -- $NIX_COMMAND"
cachix watch-exec "$CACHIX_USER" -- $NIX_COMMAND
