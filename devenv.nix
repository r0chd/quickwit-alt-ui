{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "nightly";
    targets = [ "wasm32-unknown-unknown" ];
  };

  packages = builtins.attrValues {
    inherit (pkgs)
      docker
      dioxus-cli
      wasm-bindgen-cli
      vector
      ;
  };

  processes = {
    quickwit.exec = ''
      docker run --rm \
        -v "/tmp/qwdata:/quickwit/qwdata" \
        -p 127.0.0.1:7280:7280 \
        quickwit/quickwit run
    '';

    vector.exec = ''
      vector --config ./vector.yaml
    '';
  };
}
