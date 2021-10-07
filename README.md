# gamepad-test-rs

[![Apache 2.0 License][license-shield]][license-url]

Tests gamepads using the Rust bindings for SDL2's game controller API.

## Configuration

To add a custom gamepad mapping, configure the `SDL_GAMECONTROLLERCONFIG` environment variable.

Example:
```sh
export SDL_GAMECONTROLLERCONFIG="03000000790000001100000010010000,Retrolink SNES Controller,a:b2,b:b1,back:b8,dpdown:+a1,dpleft:-a0,dpright:+a0,dpup:-a1,leftshoulder:b4,rightshoulder:b5,start:b9,x:b3,y:b0,platform:Linux,"
```

## License

Copyright &#x00A9; 2021 PotatoTech

gamepad-test is licensed under the Apache 2.0 license.
See [LICENSE][license-url] for more information.



<!-- Markdown links and images -->
[license-shield]: https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square
[license-url]: LICENSE
