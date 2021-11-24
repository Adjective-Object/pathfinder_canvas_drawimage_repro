# pathfinder_canvas_drawimage_repro

Reproduction of bug https://github.com/servo/pathfinder/issues/481

To see the difference between pathfinder_canvas and the browser implementation, open canvastest.html, and compare side by side with `cargo run`.

|                          |                                         |
| ------------------------ | --------------------------------------- |
| before                   | ![before_example](./before_compare.png) |
| after [c5da885][c5da885] | ![after_example](./after_compare.png)   |

[c5da885]: https://github.com/Adjective-Object/pathfinder/commit/c5da885fecf72b8b95679ffd0bc947b063517af9
