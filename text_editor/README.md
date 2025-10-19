## Text Editor (name work in progress)

As menitioned in the credits this is a project follwing a blog post from flenker on how to make Vim-like text editor from scratch using Rust.

Somehow this project really grew big and will take multiple days to finish, it is a great learning experience and i highly recommend following his blog post isntead of trying to figure things out from my repo.

In the end final projects will be similar but I am more lazy and quality of this tutorial is unreal.

### Current state: 

#### Day:
- 1 Caret movement + welcome message
- 2 Truncate lines + resizing
- 2 Better error handling
- 3 scrolling
- 3 complex caret movement
- 4 text viewer almost done weird bug with cluster emojies
- 5 adding deletion and insertion of characters (backspace and delete work)
- 5 advanced deletion
- 6 Enter + Tab
- 6 Added Save
- 6 Added Status Bar
- 7 Fancy Status Bar
- 7 Message Bar and small refactor for UiComponents
- 7 Added expiration to messages
- 8 Save as added
- 8 Better String handling in Line
- 9 Simple Search
- 10 Advanced Search with movement
- 10 Even more advanced search
- 12 Fixed front and back searchCommand:
- 12 Small refactor
- 13 Added Highlihting
- 13 Added Strategy Pattern for Highlighting
- 13 Added different file types
- 15 Finished 

#### Bugs: 
- *EMOJIS DONT WORK AND THEY HAVE TO GET FIXED*
- ghost image of auto resizing before applying propper view (this should be realated to line wrap)
- random crash when resizing
- pressing RIGHT goes twice *FIXED*
- emoji scrolling still doesnt work as it should
- terminal smaller then 3 rows just breaks 
- weird bug when going full screen, somehow related to cluster emojies
- Rendering order *FIXED*
- For some reason backward search doesnt work with method and works without it *FIXED*
- Have to fix foward search *FIXED*
- Scroll broken 

#### Improve:
- No need to redraw as many times as I do
- Make search work with graphemes
- Make AnnotatedString work with graphemes
- Tweak collors
- Figure out why I have to highlight whole file and not only 0..end_of_view
- Save file in different name
- Check if all highligting changes with file name change
- Remove clippy errors
- Add highlights to other file types
- Search for bugs

### Credits

https://www.flenker.blog/hecto/
