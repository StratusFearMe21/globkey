# globkey
A nodejs module for reading global keyevents, written in Rust

I'm not very good at nodejs or javascript in general, but I do know rust. So I used it to make a node module.

You use this library like so

- start the module using the `start()` function
- create a loop somewhere and call the `getKeys()` function on repeat
- when you're done, first break out of the loop you created then run the `unload()` function

And that's it. I hope you find this library helpful.
