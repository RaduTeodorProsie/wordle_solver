# wordle_solver

A small interactive Wordle solver written in Rust. It suggests guesses and filters possible answers based on feedback you provide for each guess (G = green, Y = yellow, B = grey). Feedback is case-insensitive.

## Demo video



https://github.com/user-attachments/assets/3a6c4982-58c6-478f-b432-d23c1b2165fd


## Download

- Get the prebuilt binary from the project's Releases page. No build is required — just download the appropriate executable for your platform and run it.

## Usage

- Run the downloaded binary from a terminal.
- The solver will propose top suggestions for your next guess.
- Type a guess and press Enter.
- When prompted, enter the feedback for that guess using G (green), Y (yellow) and B (grey). Upper- or lowercase is accepted.

## Notes

- If the solver repeatedly suggests the same unexpected word, verify your guess/feedback history is correct and in order. If it panics with "No matching answers for the provided feedback", one or more provided feedback entries are inconsistent with the word lists.

## Files of interest (source)

- src/main.rs — interactive CLI and program entrypoint.
- src/solver.rs — solver logic (scoring, filtering, best guess selection).
- valid-answers.txt — the secret answers list, as per the NYT.
- valid-words.txt — valid guesses list.

## License

This project is licensed under the MIT License. See LICENSE for details.
