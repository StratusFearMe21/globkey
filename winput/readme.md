[![Documentation](https://docs.rs/winput/badge.svg)](https://docs.rs/winput/)
[![Crates.io](https://img.shields.io/crates/v/winput.svg)](https://crates.io/crates/winput)

`winput` is a high-level interface to *Windows*' input system.

## Target

This crate aims to be low-level and straightforward enough to be used as a backend for other, more general crates of the genre. For this purpose, the "minimal" feature disables most of the stuff that is not really part of *Windows*' input system (things like [`Keylike`], for example, that are mostly there for convenience).

## Features

* `minimal`: This feature disables the [`Keylike`] structure as well as some shortcut functions. This feature has been made for people that want to use the straightforward api `winput` provides.
* `message_loop`: This feature enables the [`message_loop`] module that gives a way to globally retreive keyboard and mouse events from Windows' message system.

## What is left to do?

`winput` does not currently support any devices other than the mouse and the keyboard. I haven't really looked into how those work so if you know anything, feel free to submit an issue or a pull request!

## Examples

The [`Keylike`] structure allows you to synthesize keystrokes on objects that can be used as keys.

```rust
use winput::{Vk, Button};

// Synthesize keystrokes from a Virtual-Key Code
winput::press(Vk::Shift);    // press the shift key
winput::send(Vk::A);         // press then release the A key
winput::release(Vk::Shift);  // release the shift key

// Synthesize keystrokes from characters
winput::send('F');
winput::send('O');
winput::send('O');

// Synthesize keystrokes from mouse buttons
winput::send(Button::Left);

// You can synthesize keystrokes for the characters of a string
winput::send_str("Hello, world!");
```

The [`Mouse`] structure can be used to manipulate the mouse.

```rust
use winput::Mouse;

// Retrieve the position of the mouse.
let (x, y) = Mouse::position();

// Set the mouse position
//  ... in screen coordinates
Mouse::set_position(10, 10);
//  ... in normalized absolute coordinates
Mouse::move_absolute(0.5, 0.5);
//  ... relatively to the current cursor's position
Mouse::move_relative(100, 50);

// Rotate the mouse wheel (vertically)
Mouse::scroll(1.5);
//  ... or horizontally
Mouse::scrollh(-1.5);
```

For more complicated input patterns, the [`Input`] structure can be used.

```rust
use winput::{Input, Vk, Action, MouseMotion};

// There is multiple ways to create an `Input`:
let inputs = [
    // ... from a character
    Input::from_char('a', Action::Press).unwrap(),
    // ... from a Virtual-Key Code
    Input::from_vk(Vk::A, Action::Release),
    // ... from a mouse motion
    Input::from_motion(MouseMotion::Relative { x: 100, y: 100 }),

    // additional constructors are available
];

let number_of_inputs_inserted = winput::send_inputs(&inputs);

assert_eq!(number_of_inputs_inserted, 3);
```

With the `message_loop` feature (enabled by default), keyboard keystrokes and mouse inputs can be retreived.

```rust
use winput::{Vk, Action};
use winput::message_loop;

let receiver = message_loop::start().unwrap();

loop {
    match receiver.next_event() {
        message_loop::Event::Keyboard {
            vk,
            action: Action::Press,
            ..
        } => {
            if vk == Vk::Escape {
                break;
            } else {
                println!("{:?} was pressed!", vk);
            }
        },
        _ => (),
    }
}
```

[`Keylike`]: https://docs.rs/winput/latest/winput/trait.Keylike.html
[`Input`]: https://docs.rs/winput/latest/winput/struct.Input.html
[`Mouse`]: https://docs.rs/winput/latest/winput/struct.Mouse.html
[`Handler`]: https://docs.rs/winput/latest/winput/message_loop/trait.Handler.html
[`message_loop`]: https://docs.rs/winput/latest/winput/message_loop/