# Curly

Compiles to HTML.  One foot in the door into programming language development.

# Syntax

## Tags

```
head {
    title { ... }
}
body {
    div {
        ul {
            li { ... }
            li { ... }
        }
    }
}
```

## Attributes

```
input type(text) focus() value(Enter name...) {
    ...
}
```

## Attribute Shorthands

### Classes and IDs

```
div .code #special {
    ...
}
```

### Styles

```
div ^color(red) {
    ...
}
```

## Escaped Strings

```
span {
    "Hello\nWorld!"
}
```

## Raw Strings

```
span {{
    "Hello\nWorld!"
}}
```

## Implicit Bracing

```
head title "My Website"
```

### Empty Tags

```
br /
```

### Comments

```
// This is a comment
```

## An Example

Look at this document written in Curly.  Below it is the HTML file it translates into.

```
html {
    head {
        title "My website"
    }
    // comment
    body {
        h1 "Hey there!"
        h2 "Thanks for visiting my website"
        p {{
            This is my website, where I do
            websity things
        }}
        h3 .bold "My todo list"
        ul {
            li "Add more stuff to website"
            li {
                "Tell " 
                span ^color(#1411BB) "friends" 
                " about website"
            }
        }
        a href(https://google.com) "click for google"
        h3 "Talk to me!"
        p "Enter your name below and I'll know you visited my website!"
        br /
        label "Your name: "
        input type(text) /
    }
}
```

``` html
<html>
    <head>
        <title>My website</title>
    </head>
    <body>
        <h1>Hey there!</h1>
        <h2>Thanks for visiting my website</h2>
        <p>
            This is my website, where I do 
            websity things
        </p>
        <h3 class="bold">My todo list</h3>
        <ul>
            <li>Add more stuff to website</li>
            <li>
                Tell 
                <span style="color:#1411BB;">friends</span>
                 about website
            </li>
        </ul>
        <a href="https://google.com">click for google</a>
        <h3>Talk to me!</h3>
        <p>Enter your name below and I'll know you visited my website!</p>
        <br></br>
        <label>Your name: </label>
        <input type="text"></input>
    </body>
</html>
```
