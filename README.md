# Midi Controller

A rust based PWA MIDI Controller. It connects to usb or bluetooth midi devices and sends midi messages to the connected device.
The data is fully local, is never shared over the internet and is persisted even after closing the browser or rebooting your machine.

It can be installed as an app on your phone or computer.


## DEV

To run / developed locally

Install dependencies:
```bash
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli 
npm install -D tailwindcss @tailwindcss/typography
````

Compile the tailwind css:
```
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Serve the frontend:
```bash
dx serve  --hot-reload
```