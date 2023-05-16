## codepiano - pracice touch typing in CLI

Main typing mode is substitute chars in Vec<char> buffer of characters / graphemes, if next character input is the same as a lookup in the true file highlight background

## Bugs
- spacebar cant be highlighted, think of another colorscheme, maybe background
- something happens on a very last line, block it
- after backspace the cursor is behaving weird

## MVP TODO:
- Color code typed errors
- Read buffers using network or filesystem
- Mininal status bar with, metrics, help and shortcuts
- Track % completion progress on a file and on a repository level
- Save session results on exit in a JSON file or something where you can plot WPM, accuracy, top K hardest words
- probably you dont want to type the entire utf8 char set, only the ascii subset, pre-process to delete them?

## Other ideas
- Filters to ignore URL addresses, comments and similar so that you don’t have to type them, color code ignored text
- Support all highlighting that the Sublime highlighter supports
- .rc file with config on highlighting theme, cursor shape, filters