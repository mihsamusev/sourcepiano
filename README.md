## codepiano - pracice touch typing in CLI
Why codepiano?
- practice [touch typing](https://en.wikipedia.org/wiki/Touch_typing) right in your terminal with any text files you want, both plain text and source code. 
- write as you read translates to improved information retention by actively engaging with the file content.

## Bugs
- misplaced spacebar cant be highlighted, think of another colorscheme, maybe background
- something happens on a very last line, block it
- after backspace the cursor is behaving weird
- the line 11/10 can be moved to but not from

enter and spacebar as a last symbol move to next row
backspace as a first symbol moves to the end of last row

## MVP TODO:
- [x] Color code typed errors
- Read buffers using network or filesystem
- Mininal status bar with, metrics, help and shortcuts
- Track % completion progress on a file and on a repository level
- Save session results on exit in a JSON file or something where you can plot WPM, accuracy, top K hardest words
- probably you dont want to type the entire utf8 char set, only the ascii subset, pre-process to delete them?