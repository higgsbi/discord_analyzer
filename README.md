# Discord Analyzer

## Description

Discord analyzer is a simple formatting tool that takes raw json given
by Discord's "data request" tool. It allows for all messages to be dumped
into a markdown file for ease of viewing, while also giving an option
to show a simple list of channels with active conversations.

## Usage
To generate message history: `cargo run`  
For additional options: `cargo run -- --help`  

Options:  
  -p, --path \<PATH\>   
  -o, --output \<OUTPUT\>  
  -c, --channels         
  -h, --help             Print help
  -V, --version          Print version


