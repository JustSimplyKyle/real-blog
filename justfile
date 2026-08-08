clean:
    rm -rf -- target/dx/real-blog/release/web/public

dev:
    dx serve --platform web --hot-patch --features hot-patch

bundle: clean
    dx bundle --platform web -r --debug-symbols false

deploy: bundle
    nix shell nixpkgs#wrangler --command wrangler pages deploy
