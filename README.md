# Unlock the Box

Unlock the Box is a simple Rust command-line program that simulates unlocking a box by entering a correct code. The program continuously prompts the user to enter a code and checks if the entered code matches the predefined code. If the correct code is entered, the box is unlocked; otherwise, it resets and allows the user to try again.

## Features

- Prompts the user to enter a code.
- Validates the entered code.
- Unlocks the box if the correct code is entered.
- Resets if an incorrect code is entered.

## Installation

To run this program, you need to have Rust installed on your machine. If you don't have Rust installed, you can download and install it from [here](https://www.rust-lang.org/tools/install).

## Usage

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/unlock-the-box.git
    cd unlock-the-box
    ```

2. Compile the program:
    ```sh
    cargo build --release
    ```

3. Run the program:
    ```sh
    cargo run
    ```

4. Follow the on-screen instructions to enter the code and unlock the box.

## Code Overview

### State Enum

The `State` enum defines three states for the box:
- `Locked`: The initial state where the program waits for user input.
- `Unlocked`: The state when the correct code is entered.
- `Failed`: The state when an incorrect code is entered, prompting the user to try again.

### Main Function

The main function contains the core logic for the program:
- Prompts the user to enter the code.
- Reads the input and trims any trailing newline characters.
- Checks if the entered code matches the predefined code (`123`).
    - If the code matches, the state changes to `Unlocked` and displays a success message.
    - If the code does not match or the entry is incorrect, the state changes to `Failed`, clears the input, and resets to the `Locked` state.

### Flow

1. **Locked State**: The program prompts the user for input and checks if the entered code matches the predefined code.
2. **Unlocked State**: If the correct code is entered, the program displays a success message and terminates.
3. **Failed State**: If an incorrect code is entered, the program displays a failure message, clears the entered code, and returns to the `Locked` state.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
