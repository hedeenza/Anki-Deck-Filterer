# Anki Deck File Zipper

## Use Case:
- Tab-separated values (.tsv) files used for Anki can become wide very quickly, especially when sentences become longer and more fields are introduced (audio files, sources, etymology, notes, etc.). When using automation tools, there are times I end up with near-duplicates or sentences I didn't want to include in the deck. This tool allows a much narrower (and easier to read file) to filter your import file down to just the lines you want.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Running the CLI
`$ ./anki_deck_filterer <Primary_File> <Filtering_File> <Column_to_Filter_on> <Output_File>`

- `<Primary_File>` = File containing the sentences in the target language, translations, audio file names, etc.
- `<Filtering_File>` = File containing the values used to filter the Main File. The lines *DO NOT* need to be in the same order as they are in the <Primary_File>, allowing alphabetization of the filtering file. Changing the order of lines in the <Filtering_File> will not affect the order of lines in the <Primary_File>.
- `<Column_to_Filter_on>` = The column name of the column in the `<Primary_File>` the values in the `<Filtering_File>` will be compared to. 
- `<Output_File>` = The name of the filtered file produced by the program.
- Ensure the file-filtering program has executable permissions.
- Ensure the `<Primary_File>` contains column names.
- Ensure the text in <Filtering_File> *exactly* matches (capitalization, spacing, etc.) the text in the appropriate <Primary_File> column.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Anki Deck Filterer
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias af="anki_deck_filterer"
```

## License

This program is distributed under the terms of a GNU GPLv3 license. See LICENSE.md for details.
