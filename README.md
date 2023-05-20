## sourcepiano - pracice touch typing anything in CLI
Why sourcepiano?
- practice [touch typing](https://en.wikipedia.org/wiki/Touch_typing) right in your terminal with any text files you want, both plain text and source code. 
- write as you read translates to improved information retention by actively engaging with the file content.

## Getting started
```bash
sourcepiano <file>
```

```bash
sourcepiano <url>
```

## Bugs
- [ ] enter and spacebar as a last symbol move to next row unless its alredy last row
- [ ] backspace as a first symbol moves to the end of previous row unless its already first row
- [ ] the line 11/10 can be moved to but not from
- [ ] misplaced spacebar cant be highlighted, think of another colorscheme, maybe background


## MVP TODO:
- [x] Color code typed errors
- move colorscheme to the config file like `.codepianorc`
- Mininal status bar with, navigation, metrics and shortcuts

### IO
- [x] Read a single file into buffer
- [ ] Read buffers using network
- [ ] Preprocessors - config with filters on text file like, remove all non ascii chars, remove links, remove images, etc
- [ ] Save session results on exit in a JSON file that can be used to visualize metrics

### Metrics
- [ ] Track % completion progress on a document
- [ ] Track precision on a row level and on a document level
- [ ] Track speed on a row level and on a document level
- [ ] Rank symbols and tokens by difficulty