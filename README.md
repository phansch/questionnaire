# questionnaire

A program to ask questions on the command line and collect the results in
JSON files.

This is not a library.

## Usage

```console
questionnaire questions.json
```

Where `questions.json` has to have the following structure:

```json
[
    {
        "name": "An identifier for the question",
        "prompt": "The actual question",
        "answer": {
            "kind": "see below",
            "choices": [0, 1, 2, 3]
        }
    }
]
```

`answer.kind` can be one of the following:

* `select` will ask the user to select something from a given set of options,
  defined in `answer.choices`.
* `input` will ask for basic text
* `editor` will open the `$EDITOR` to enter text

```json
[
    {
        "name": "some_identifier",
        "prompt": "The actual question",
        "answer": {
            "kind": "select",
            "choices": [0, 1, 2, 3]
        }
    },
    {
        "name": "some_identifier",
        "prompt": "The actual question",
        "answer": {
            "kind": "input"
        }
    },
    {
        "name": "some_identifier2",
        "prompt": "The actual question",
        "answer": {
            "kind": "editor"
        }
    }
]
```
