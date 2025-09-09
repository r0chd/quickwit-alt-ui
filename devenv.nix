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

  services.nginx = {
    enable = true;
    httpConfig = ''
      server {
          listen 8080;
          server_name localhost;
          root ${builtins.getEnv "PWD"}/dist;
          index index.html;

          # Serve static files
          location / {
              try_files $uri $uri/ /index.html;
              add_header Access-Control-Allow-Origin "*";
              add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
              add_header Access-Control-Allow-Headers "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range";
          }

          # Proxy API requests to Quickwit
          location /api/ {
              proxy_pass http://localhost:7280/;
              proxy_set_header Host $host;
              proxy_set_header X-Real-IP $remote_addr;
              proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
              proxy_set_header X-Forwarded-Proto $scheme;

              # CORS headers for preflight and proxied requests
              add_header Access-Control-Allow-Origin "*" always;
              add_header Access-Control-Allow-Methods "GET, POST, OPTIONS, DELETE, PUT" always;
              add_header Access-Control-Allow-Headers "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization" always;

              # Handle preflight requests
              if ($request_method = 'OPTIONS') {
                  add_header Access-Control-Allow-Origin "*";
                  add_header Access-Control-Allow-Methods "GET, POST, OPTIONS, DELETE, PUT";
                  add_header Access-Control-Allow-Headers "DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization";
                  add_header Access-Control-Max-Age 1728000;
                  add_header Content-Type 'text/plain; charset=utf-8';
                  add_header Content-Length 0;
                  return 204;
              }
          }
      }
    '';
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
