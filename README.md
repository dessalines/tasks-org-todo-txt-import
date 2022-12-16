# Tasks-Org-Todo-Txt-Import

Import your [todo.txt](http://todotxt.org/) tasks into the [tasks.org](https://github.com/tasks/tasks) format.

## Instructions

- Add your `todo.txt` file to this directory.
- Go to your Tasks app, export a backup.
- Run the following:

`cargo run > out.txt`

The output will have:

```
Tasks
---
{tag JSON},
...
Tags
---
{task JSON},
...
```

- Copy paste those portions into the `"tags": [...]` and `"tasks": [...]` of your backup.
