clean:
    rm -rf -- target/dx/real-blog/release/web/public

dev:
    dx serve --hot-patch --features hot-patch

bundle: clean
    dx bundle -r --debug-symbols false

deploy: bundle
    nix shell nixpkgs#wrangler --command wrangler pages deploy
