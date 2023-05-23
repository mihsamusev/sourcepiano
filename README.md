## sourcepiano - pracice touch typing anything in CLI
Why `sourcepiano`?
- Exercise [touch typing](https://en.wikipedia.org/wiki/Touch_typing) in your terminal with plain text and source code. 
- Capture metrics to drive your practice.
- Study source code and text by re-writting it. Actively engaging with the file content translates to improved information retention.

## Getting started
```bash
sourcepiano <file>
sourcepiano <url> 
curl <url> | sourcepiano
```

## MVP TODO:
### Bugs
- move to `crossterm`
- disable terminal scrolling
- horizontal wrap / scroll is buggy

### UI
- [x] Color code typed errors
- [ ] config file like `.codepianorc`
    - [ ] correct color
    - [ ] incorrect color
    - scroll step
- [ ] Mininal status bar with, navigation, metrics and shortcuts

### IO
- [x] Read a single file into buffer
- [ ] Read from pipe
- [ ] Read buffers using network
- [ ] Preprocessors - config with filters on text file like, remove all non ascii chars, remove links, remove images, etc
- [ ] Save session results on exit in a JSON file that can be used to visualize metrics

### Metrics
- [ ] Track % completion progress on a document
- [ ] Track precision on a row level and on a document level
- [ ] Track speed on a row level and on a document level
- [ ] Rank symbols and tokens by difficulty