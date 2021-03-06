# fi

Cli for personal finance

![fi history](./assets/history.gif)


### Usage

Pull data from Notion tables
```
$ fi pull
```

Show history for given currency
```
$ fi history [--currency | -c] <currency>
```

Show latest for given currency accounts
```
$ fi sum [--currency | -c] <currency>
```

Show sum of all accounts converted to given currency
```
$ fi networth [--currency | -c] <currency>
```

and some more…

```
USAGE:
    fi <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete      Delete all table rows
    help        Prints this message or the help of the given subcommand(s)
    history     Display history of accounts
    networth    Display net worth in given currency
    pull        Pull account and snapshot data from notion table
    sum         Display latest sum for given currency
```

If you:

- feel like keeping track of your personal finances in Notion
- are weird enough that you want to use cli to view the data
- don't mind setting up free Postgres DB on Heroku (see `.env.example`)

then you can use it, too.

---

Here's a sample data recorded in Notion.
### [Test data on Notion](https://www.notion.so/dejima/Fi-Test-Data-48a9e5b2a9324762a76fd41bd83ca4c0)

I am hosting "Notion API Worker" myself using Cloudflare in order to access Notion table data from cli. (https://notion-api.mkd.workers.dev).
If your notion page/table is public, you can also use this endpoint to query your data.

This is a neat tool that allows you to access your Notion content, created by [these nice people](https://github.com/splitbee/notion-api-worker)

See the [docs](https://github.com/splitbee/notion-api-worker#authentication-for-private-pages) for how to set the token for private pages. For public pages there's no need for authorization.
