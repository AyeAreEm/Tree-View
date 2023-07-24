# Tree View
Currently in pre-alpha

be wary of Directory Man!
<p align="center">
    <img src="sample.png" width=500/>
</p>

## Features
- ✅ Add new `master` directories and switch between them
- ✅ Search for a file in the current directory
- ✅ Show files and folders in a vertical tree diagram
- ✅ Get properties of a file 
- ✅ Open a file in its respective app 
- ✅ Option to open directory in Windows File Explorer
- ✅ Option to open directory in terminal
- Add a new file or folder
- Remove a file or folder
- Link a chosen directory to a chosen file (main feature of this. unavailable in regular file explorer)
- [Settings Section](#settings-section) 

## Todos
- ✅ Don't display big folders such as node_modules and .git
- Add settings section with customisation (i.e. change background, specific folder's to ignore, etc)
- Add website / cloud verison so that files are accessible everywhere [note: this will preferably not use an account system but something still as secure hopefully]
- Don't display dot files such as .DS_Store and .gitignore? (maybe)

---

## Installation
As of this moment, the only way to run this for yourself is by cloning the repo run these commands in the directory:

`npm i`
`npm run tauri dev`

This project has not been tested in its build version and is not recommended until Tree View is in 1.0

## How to use
- Add your master directory with the + button at the top left.
- Remove (not delete) a master directory with the - button
- Load it by clicking on the selection options - top left
- Hover over files / folders to see their name
- Search for files and folders with the name of them [note: all entities that has the search term will appear]
- Click on the file / folder once to open it in its respective app
- Double click to get more info - size, name, location, options to open in the system file explorer and in the terminal, etc.

### Settings Section
- Background options
- Icon options? (might be to much of a hassle)
- [Upgrade package](#upgrade-package) (subscription / it'd be greatly appreciated if you could help me get a cup of coffee 🥺)

#### Upgrade package includes: 
- Cloud system (store files and such - allows users to store and port files to other devices)
- More customisation (specific folder's to ignore, etc.)
- Get to try upcoming features?

---

## How it works
Using Rust (Tauri) as the backend and Javascript (Svelte and D3) as the frontend. 

Rust recursively gets the files and folders in a chosen directory, supplies it to the frontend where it is drawn into a tree diagram.

### But how does linking unrelated folders and files work?
Currently, haven't added it yet but I believe I can create my own dot file (.gitignorre, .git, .env, etc).
It'll probably be named `.treeview` or something along those lines

Maybe just draw a line connecting them on the frontend and save it to localstorage? not sure.

---

## Resources and references
### All assets used

[SVG Images](https://www.svgrepo.com)

### All the third party libraries used

[WalkDir](https://docs.rs/walkdir/latest/walkdir/) (Rust)

[D3](https://d3js.org) (Javascript)

[Open](https://docs.rs/open/latest/open/) (Rust)

[Rand](https://docs.rs/rand/latest/rand/) (Rust)

### Use of WalkDir
This is crate makes the nightmare of traversing a directory recursively incredibly easy
Initally, I wanted to write it by myself but when researching, I stumbled upon [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html)
In there under Directory Traversal, they even used this crate so I guess it's practically a staple in the Rust directory traversal scene.

```rust
let content: Vec<_> = WalkDir::new(directory)
                    .into_iter()
                    .filter_map(|f| f.ok())
                    .map(|f| f.path().to_owned())
                    .collect();
```
That's it. Done. Amazing. Using this, I had to convert the path buffer to a string which was weirdly hard but I got there in the end. I needed it to be a regular string and not a `.display()` or `.into_os_string().into_string()` mainly because I didn't want to deal with the hassle and because I needed to spend strings to the frontend.

### Use of D3
I wanna throw a washing machine at the nearest wall
The learning curve for D3 is insane. The docs are _decently_ documented but it feels like they assume you know what each thing returns or what methods are chainable.
To be fair, this is the first time I'm using D3, and it's gonna be the major part of this project so unless I can find a smaller library or write code that can draw diagrams from scratch by myself, I should keep quite, suck it up, and just git gud
I will say, after spending a quater of a day, it gets easier to notice similarities and a sort of "system" if that makes sense.

BUT, working with svg's is another nightmare. And the fact that there is not a lot of people using svelte and d3, there's almost no examples.

```js
// I will admit, two lines of code to setup a tree diagram is really good. but it took me a surprising amount of time to write these two lines
$: root = d3.stratify().path((d) => d)(paths);
$: treeLayout = d3.tree().size([width, height - 40])(root);
```

```js
// this is to draw the lines between the different nodes
{#each root.links() as link}
    <line x1={link.source.x} y1={link.source.y + recHeight} x2={link.target.x} y2={link.target.y} stroke="#adadad"></line>
{/each}
```
### Use of Open
Perfect and simple. This crate lets you open a file in the os's default app (whatever you set that default to be) or in a specific app.

```rust
open::that(location).unwrap();
```
That's it. literally that's it. Chef's kiss.
Although, I might need to use it more since we are mostly using rust's built in Command library since we need to run commands with certain apps when opening them (i.e. terminal and file explorer - the main ones 😅)

### Use of Rand
I don't think there's a lot to say about this one. It's just a random number generator.
Rust doesn't have a rng included in its stdlib.
The reason for this is each window needs a unique label when it's created, so random number yk.

```rust
let rng = rand::thread_rng();
```
