# wordle_solver

A small, fast interactive Wordle solver (Rust) that suggests guesses and filters possible answers based on feedback. Feedback accepts G (green), Y (yellow), and B (grey) in either case.

---

🎞️ Demo

- Watch: https://github.com/user-attachments/assets/3a6c4982-58c6-478f-b432-d23c1b2165fd

---

⬇️ Download

- Download a prebuilt binary from the Releases page and run it — no build required.

---

🚀 Quick usage

1. Run the downloaded binary from a terminal.
2. The solver will propose one or more suggested guesses.
3. Type a guess and press Enter.
4. Enter feedback for that guess using G (green), Y (yellow), and B (grey). Examples: `ggbby`, `GyBbY`, `BBBBB`.

Example session

> Top suggestion: roate

What's your guess?

roate

What's the feedback for roate?

ggbby

The solver will update its possible answers and show new suggestions.

---

🧭 Notes & troubleshooting

- Feedback is case-insensitive.
- If the solver repeatedly suggests the same unexpected word, confirm your guess/feedback history is correct and in chronological order.
- A panic saying "No matching answers for the provided feedback" means one or more of your provided feedback entries are inconsistent with the word lists (valid-answers.txt).

---

📁 Files of interest (source)

- src/main.rs — CLI and entry point
- src/solver.rs — solver logic (scoring, filtering, best-guess selection)
- valid-answers.txt — possible secret answers (NYT list)
- valid-words.txt — valid guesses

---

🤝 Contributing

Issues and pull requests welcome. Please include a short reproduction for bugs.

---

📜 License

MIT — see LICENSE
