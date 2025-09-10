# Skuff

**skuff** (Norwegian for drawer) is a simple time-recording tool for developers
who’d rather stay in the terminal than wrestle with corporate timesheet
software.

Many companies use complex systems like SAP for logging hours. If you are like
me, you might want to avoid opening these systems multiple times a day just to
say you are "in" or "out". Instead, you might keep a little skuffe-regnskap
(literally drawer-accounting): jotting down your hours on a note or in a drawer
until the end of the week or month, and only then transferring them into the
official system. skuff brings that drawer into the terminal — the one app
that’s always open on a developer’s machine.

Start your day with:
```bash
skuff in
```

And end your day with:
```bash
skuff out
```

That's it, really. When you are ready to transfer your hours to the official
tool, you can summarize your hours with:

```bash
skuff log
```
