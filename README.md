# TUI Tree Widget

The Tree Data Structure widget for the ratatui library is engineered to enable easy and fast manipulation of tree data structures using arrow keys, streamlining operations similar to those found in the older Google Tasks, as well as in modern tools like Quire and Microsoft Project.

Key functionalities:
- **Quick Node Reordering**: Utilize the shift-up and shift-down arrow keys for rapid repositioning of nodes within the tree, facilitating swift adjustments to your data structure.
- **Efficient Hierarchical Changes**: Employ the shift-right-arrow key to effortlessly nest nodes into subfolders, or use the shift-left-arrow key to quickly move nodes out of subfolders, making structural modifications straightforward and fast.
- **Simple Node Addition**: Add new nodes to your tree with the ease of pressing the shift-enter key, allowing for immediate expansion of your data structure.

This is a work in progress, and the widget is not yet fully functional. The current version is a prototype, and the widget is not yet ready for use in production.

![Screenshot](media/screenshot.png)

## Remarks
This library is developed with Visual Studio Code and WSL2.
Some configurations are specific to this environment, such as the launch.json file for debugging.

## Usage

### Run the example 
cargo run --examples example

### Debugging with vscode and wsl2
Directly starting the Debugger in vscode with lldb or gdb has issues with the display of the terminal.
A workaround is to run the program in the terminal using __cargo run --examples example__ and attach the debugger to the running process. In order to do this a vscode launch configuration has been added (see the .vscode folder).
